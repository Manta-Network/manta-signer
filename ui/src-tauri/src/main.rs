// Copyright 2019-2021 Manta Network.
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

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use manta_signer::{
    config::Config,
    secret::{account_exists, create_account, Authorization, Authorizer, Password},
    service::Service,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{
    async_runtime::{channel, spawn, Receiver, RwLock, Sender},
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
};

/// User
pub struct User {
    /// Main Window
    window: Window,

    /// Password Receiver
    password: Receiver<Password>,
}

impl User {
    /// Builds a new [`User`] from `window` and `password`.
    #[inline]
    pub fn new(window: Window, password: Receiver<Password>) -> Self {
        Self { window, password }
    }
}

impl Authorizer for User {
    #[inline]
    fn authorize<T>(&mut self, prompt: T) -> Authorization
    where
        T: Serialize,
    {
        self.window.emit("authorize", prompt).unwrap();
        self.window.center().unwrap();
        self.window.show().unwrap();
        Box::pin(async move {
            let password = self.password.recv().await;
            self.window.hide().unwrap();
            password
        })
    }
}

/// Password Storage Type
type PasswordStoreType = Arc<RwLock<Option<Sender<Password>>>>;

/// Password Storage Handle
pub struct PasswordStoreHandle(PasswordStoreType);

impl PasswordStoreHandle {
    /// Builds a new password storage system, waiting on the receiver to receive it's first
    /// message.
    #[inline]
    pub async fn setup(self) -> Receiver<Password> {
        let (sender, mut receiver) = channel(1);
        *self.0.write().await = Some(sender);
        receiver.recv().await;
        receiver
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

    /// Loads a new password into the state.
    #[inline]
    pub async fn load(&self, password: String) {
        if let Some(state) = &*self.0.read().await {
            state.send(Password::Known(password)).await.unwrap();
        }
    }

    /// Clears the password state.
    #[inline]
    pub async fn clear(&self) {
        if let Some(state) = &*self.0.read().await {
            state.send(Password::Unknown).await.unwrap();
        }
    }
}

/// Loads the current `password` into storage from the UI.
#[tauri::command]
async fn load_password(
    password_store: State<'_, PasswordStore>,
    password: String,
) -> Result<(), ()> {
    password_store.load(password).await;
    Ok(())
}

/// Removes the current password from storage.
#[tauri::command]
async fn clear_password(password_store: State<'_, PasswordStore>) -> Result<(), ()> {
    password_store.clear().await;
    Ok(())
}

/// Connection Event
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ConnectEvent {
    /// Create Account
    CreateAccount,

    /// Setup Authorization
    SetupAuthorization,
}

/// Starts the first round of communication between the UI and the signer.
#[tauri::command]
async fn connect(window: Window, config: State<'_, Config>) -> Result<ConnectEvent, ()> {
    match account_exists(&config.root_seed_file).await {
        Ok(true) => Ok(ConnectEvent::SetupAuthorization),
        _ => {
            window.set_always_on_top(true).unwrap();
            window.center().unwrap();
            window.show().unwrap();
            Ok(ConnectEvent::CreateAccount)
        }
    }
}

/// Sends the mnemonic to the UI for the user to memorize.
#[tauri::command]
async fn get_mnemonic(config: State<'_, Config>, password: String) -> Result<String, ()> {
    Ok(create_account(&config.root_seed_file, password)
        .await
        .map_err(move |_| ())?
        .into_phrase())
}

/// Ends the first round of communication between the UI and the signer.
#[tauri::command]
async fn end_connect(window: Window, password_store: State<'_, PasswordStore>) -> Result<(), ()> {
    window.hide().unwrap();
    password_store.clear().await;
    Ok(())
}

/// Runs the main Tauri application.
fn main() {
    let config =
        Config::try_default().expect("Unable to generate the default server configuration.");

    let mut app = tauri::Builder::default()
        .system_tray(
            SystemTray::new()
                .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("exit", "Quit"))),
        )
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } if id == "exit" => app.exit(0),
            _ => {}
        })
        .manage(PasswordStore::default())
        .manage(config)
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let password_store = app.state::<PasswordStore>().handle();
            let config = app.state::<Config>().inner().clone();
            spawn(async move {
                // FIXME: We are duplicating calls to `setup`.
                config.setup().await.unwrap();
                Service::build(config, User::new(window, password_store.setup().await))
                    .serve()
                    .await
                    .expect("Unable to build manta-signer service.");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect,
            get_mnemonic,
            end_connect,
            clear_password,
            load_password,
        ])
        .build(tauri::generate_context!())
        .expect("Error while building UI.");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(move |_, _| {})
}
