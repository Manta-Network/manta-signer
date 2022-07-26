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

use alloc::sync::Arc;
use core::time::Duration;
use manta_signer::{
    config::{Config, Setup},
    secret::{Authorizer, Password, PasswordFuture, Secret, SecretString, UnitFuture},
    serde::Serialize,
    service::{self, receiving_key_to_base58, ReceivingKeyRequest, Server},
    tokio::time::sleep,
};
use tauri::{
    async_runtime::{channel, spawn, Mutex, Receiver, Sender},
    CustomMenuItem, Manager, RunEvent, State, SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
    WindowEvent,
};

/// User
pub struct User {
    /// Main Window
    window: Window,

    /// Password Receiver
    password: Receiver<Password>,

    /// Password Retry Sender
    retry: Sender<bool>,

    /// Waiting Flag
    waiting: bool,
}

impl User {
    /// Builds a new [`User`] from `window`, `password`, and `retry`.
    #[inline]
    pub fn new(window: Window, password: Receiver<Password>, retry: Sender<bool>) -> Self {
        Self {
            window,
            password,
            retry,
            waiting: false,
        }
    }

    /// Emits a `message` of the given `kind` to the window.
    #[inline]
    fn emit<T>(&self, kind: &'static str, message: &T)
    where
        T: Serialize,
    {
        self.window.emit(kind, message).unwrap()
    }

    /// Sends a the `retry` message to have the user retry the password.
    #[inline]
    async fn should_retry(&mut self, retry: bool) {
        self.retry
            .send(retry)
            .await
            .expect("Failed to send retry message.");
    }

    /// Requests password from user, sending a retry message if the previous password did not match
    /// correctly.
    #[inline]
    async fn request_password(&mut self) -> Password {
        if self.waiting {
            self.should_retry(true).await;
        }
        let password = self
            .password
            .recv()
            .await
            .expect("Failed to receive retry message.");
        self.waiting = password.is_known();
        password
    }

    /// Sends validation message when password was correctly matched.
    #[inline]
    async fn validate_password(&mut self) {
        self.waiting = false;
        self.should_retry(false).await;
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
            // NOTE: We have to wait here until the UI listener is registered.
            sleep(Duration::from_millis(500)).await;
            window
                .emit("connect", setup)
                .expect("The `connect` command failed to be emitted to the window.");
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

/// Password Storage Channel
struct PasswordStoreChannel {
    /// Password Sender
    password: Sender<Password>,

    /// Retry Receiver
    retry: Receiver<bool>,
}

/// Password Storage Type
type PasswordStoreType = Arc<Mutex<Option<PasswordStoreChannel>>>;

/// Password Storage Handle
pub struct PasswordStoreHandle(PasswordStoreType);

impl PasswordStoreHandle {
    /// Constructs the opposite end of `self` for the password storage handle.
    #[inline]
    pub async fn into_channel(self) -> (Receiver<Password>, Sender<bool>) {
        let (password, receiver) = channel(1);
        let (sender, retry) = channel(1);
        *self.0.lock().await = Some(PasswordStoreChannel { password, retry });
        (receiver, sender)
    }
}

/// Password Storage
#[derive(Default)]
pub struct PasswordStore(PasswordStoreType);

impl PasswordStore {
    /// Returns a handle for setting up a [`PasswordStore`].
    #[inline]
    pub fn handle(&self) -> PasswordStoreHandle {
        PasswordStoreHandle(self.0.clone())
    }

    /// Loads the password store with `password`, returning `true` if the password was correct.
    #[inline]
    pub async fn load(&self, password: SecretString) -> bool {
        if let Some(store) = &mut *self.0.lock().await {
            let _ = store.password.send(Password::from_known(password)).await;
            store.retry.recv().await.unwrap()
        } else {
            false
        }
    }

    /// Loads the password with `password`, not requesting a retry.
    #[inline]
    pub async fn load_exact(&self, password: SecretString) {
        if let Some(store) = &mut *self.0.lock().await {
            let _ = store.password.send(Password::from_known(password)).await;
        }
    }

    /// Clears the password from the store.
    #[inline]
    pub async fn clear(&self) {
        if let Some(store) = &mut *self.0.lock().await {
            let _ = store.password.send(Password::from_unknown()).await;
        }
    }
}

///
type ServerStoreType = Arc<Mutex<Option<Server<User>>>>;

///
pub struct ServerStoreHandle(ServerStoreType);

impl ServerStoreHandle {
    ///
    #[inline]
    pub async fn set(&self, server: Server<User>) {
        *self.0.lock().await = Some(server)
    }
}

///
#[derive(Default)]
pub struct ServerStore(ServerStoreType);

impl ServerStore {
    ///
    #[inline]
    pub fn handle(&self) -> ServerStoreHandle {
        ServerStoreHandle(self.0.clone())
    }
}

/// Sends the current `password` into storage from the UI.
#[tauri::command]
async fn send_password(
    password_store: State<'_, PasswordStore>,
    password: String,
) -> Result<bool, ()> {
    Ok(password_store.load(Secret::new(password)).await)
}

/// Stops the server from prompting for the password.
#[tauri::command]
async fn stop_password_prompt(password_store: State<'_, PasswordStore>) -> Result<(), ()> {
    password_store.clear().await;
    Ok(())
}

/// Sends all receiving keys to the UI.
#[tauri::command]
async fn receiving_keys(server: State<'_, ServerStore>) -> Result<Vec<String>, String> {
    Ok(server
        .0
        .lock()
        .await
        .clone()
        .unwrap()
        .receiving_keys(ReceivingKeyRequest::GetAll)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|key| receiving_key_to_base58(&key))
        .collect())
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
                    .add_item(CustomMenuItem::new("account", "Account"))
                    .add_item(CustomMenuItem::new("exit", "Quit")),
            ),
        )
        .on_system_tray_event(move |app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "about" => app.get_window("about").unwrap().show().unwrap(),
                    "account" => app.get_window("main").unwrap().emit("account", "").unwrap(),
                    "exit" => app.exit(0),
                    _ => {}
                }
            }
        })
        .manage(ServerStore::default())
        .manage(PasswordStore::default())
        .manage(config)
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let config = app.state::<Config>().inner().clone();
            let password_store = app.state::<PasswordStore>().handle();
            let server_store = app.state::<ServerStore>().handle();
            spawn(async move {
                let (password, retry) = password_store.into_channel().await;
                let server = service::setup(&config, User::new(window, password, retry))
                    .await
                    .expect("Unable to setup manta-signer service.");
                server_store.set(server.clone()).await;
                service::start(&config, server)
                    .await
                    .expect("Unable to build manta-signer service.");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_password,
            stop_password_prompt,
            receiving_keys
        ])
        .build(tauri::generate_context!())
        .expect("Error while building UI.");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|app, event| match event {
        RunEvent::Ready => app.get_window("about").unwrap().hide().unwrap(),
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            api.prevent_close();
            match label.as_str() {
                "about" => app.get_window(&label).unwrap().hide().unwrap(),
                "main" => app.exit(0),
                _ => unreachable!("There are no other windows."),
            }
        }
        _ => (),
    })
}
