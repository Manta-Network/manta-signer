// Copyright 2019-2022 Manta Network.
// This file is part of manta-signer.
//
// manta-signer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-signer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-signer. If not, see <http://www.gnu.org/licenses/>.

//! Manta Signer UI

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate alloc;

use manta_signer::{
    config::{Config, Setup},
    secret::{
        password_channel, Authorizer, Password, PasswordFuture, PasswordReceiver, PasswordSender,
        Secret, UnitFuture,
    },
    serde::Serialize,
    service::Server,
    storage::Store,
    tokio::time::sleep,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tauri::{
    async_runtime::spawn, CustomMenuItem, Manager, RunEvent, Runtime, State, SystemTray,
    SystemTrayEvent, SystemTrayMenu, Window, WindowEvent,
};

// UI_READY set by invoking ui_ready from react front end
// Atomic so we can modify without unsafe blocks
static UI_READY: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn ui_ready() {
    UI_READY.store(true, Ordering::Relaxed);
}

macro_rules! ui_ready {
    ($body:block, $timeout_d:expr, $failure:block) => {{
        let time_start = Instant::now();
        let timeout = Duration::from_millis($timeout_d);
        loop {
            if UI_READY.load(Ordering::Relaxed) {
                $body
                break;
            }
            if time_start.elapsed() >= timeout {
                $failure
            }
        }
    }};
}

/// User
pub struct User {
    /// Main Window
    window: Window,

    /// Password Receiver
    password_receiver: PasswordReceiver,

    /// Waiting Flag
    waiting: bool,
}

impl User {
    /// Builds a new [`User`] from `window` and `password_receiver`.
    #[inline]
    pub fn new(window: Window, password_receiver: PasswordReceiver) -> Self {
        Self {
            window,
            password_receiver,
            waiting: false,
        }
    }

    /// Emits a `message` of the given `kind` to the window.
    #[inline]
    fn emit<T>(&self, kind: &'static str, message: &T)
    where
        T: Serialize,
    {
        self.window
            .emit(kind, message)
            .expect("Unable to emit message to the window.")
    }

    /// Requests password from user, sending a retry message if the previous password did not match
    /// correctly.
    #[inline]
    async fn request_password(&mut self) -> Password {
        if self.waiting {
            self.password_receiver.should_retry(true).await;
        }
        let password = self.password_receiver.password().await;
        self.waiting = password.is_known();
        password
    }

    /// Sends validation message when password was correctly matched.
    #[inline]
    async fn validate_password(&mut self) {
        self.waiting = false;
        self.password_receiver.should_retry(false).await;
    }
}

impl Authorizer for User {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move { self.request_password().await })
    }

    #[inline]
    fn setup<'s>(&'s mut self, setup: &'s Setup) -> UnitFuture<'s> {
        let window = self.window.clone();
        Box::pin(async move {
            ui_ready!(
                {
                    window
                        .emit("connect", setup)
                        .expect("The `connect` command failed to be emitted to the window.");
                },
                10000,
                { panic!() }
            );
        })
    }

    #[inline]
    fn wake<T>(&mut self, prompt: &T) -> UnitFuture
    where
        T: Serialize,
    {
        self.emit("authorize", prompt);
        Box::pin(async move {})
    }

    #[inline]
    fn sleep(&mut self) -> UnitFuture {
        Box::pin(async move { self.validate_password().await })
    }
}

/// Password Store
pub type PasswordStore = Store<PasswordSender>;

/// Server Store
pub type ServerStore = Store<Server<User>>;

/// Sends the current `password` into storage from the UI.
#[tauri::command]
async fn send_password(
    password_store: State<'_, PasswordStore>,
    password: String,
) -> Result<bool, ()> {
    if let Some(store) = &mut *password_store.lock().await {
        Ok(store.load(Secret::new(password)).await)
    } else {
        Ok(false)
    }
}

/// Stops the server from prompting for the password.
#[tauri::command]
async fn stop_password_prompt(password_store: State<'_, PasswordStore>) -> Result<(), ()> {
    if let Some(store) = &mut *password_store.lock().await {
        store.clear().await;
    }
    Ok(())
}

/// Returns the window with the given `label` from `app`.
///
/// # Panics
///
/// This function panics if the window with the given `label` was unreachable.
#[inline]
pub fn window<R, M>(app: &M, label: &str) -> Window<R>
where
    R: Runtime,
    M: Manager<R>,
{
    match app.get_window(label) {
        Some(window) => window,
        _ => panic!("Unable to get {:?} window handler.", label),
    }
}

/// Runs the main Tauri application.
fn main() {
    let config =
        Config::try_default().expect("Unable to generate the default server configuration.");

    let mut app = tauri::Builder::default()
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("about", "About"))
                    .add_item(CustomMenuItem::new("exit", "Quit")),
            ),
        )
        .on_system_tray_event(move |app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "about" => window(app, "about").show().expect("Unable to show window."),
                    "exit" => app.exit(0),
                    _ => {}
                }
            }
        })
        .manage(PasswordStore::default())
        .manage(ServerStore::default())
        .setup(|app| {
            let window = window(app, "main");
            let password_store = app.state::<PasswordStore>().inner().clone();
            let server_store = app.state::<ServerStore>().inner().clone();
            spawn(async move {
                let (password_sender, password_receiver) = password_channel();
                password_store.set(password_sender).await;
                let server = Server::build(config, User::new(window, password_receiver))
                    .await
                    .expect("Unable to build manta-signer server.");
                server_store.set(server.clone()).await;
                server
                    .start()
                    .await
                    .expect("Unable to build manta-signer service.");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_password,
            stop_password_prompt,
            ui_ready,
        ])
        .build(tauri::generate_context!())
        .expect("Error while building UI.");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|app, event| match event {
        RunEvent::Ready => window(app, "about").hide().expect("Unable to hind window."),
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            api.prevent_close();
            match label.as_str() {
                "about" => window(app, "about").hide().expect("Unable to hide window."),
                "main" => app.exit(0),
                _ => unreachable!("There are no other windows."),
            }
        }
        _ => (),
    })
}
