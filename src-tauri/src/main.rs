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

use manta_signer::{account_exists, create_account, PasswordStore, Service};
use serde::{Deserialize, Serialize};
use tauri::{
    async_runtime::spawn, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent,
    SystemTrayMenu, Window,
};

/// Loads the current `password` into storage from the UI.
#[tauri::command]
async fn load_password(state: State<'_, PasswordStore>, password: String) -> Result<(), ()> {
    state.load(password).await;
    Ok(())
}

/// Removes the current password from storage.
#[tauri::command]
async fn clear_password(state: State<'_, PasswordStore>) -> Result<(), ()> {
    state.clear().await;
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
async fn connect(window: Window) -> ConnectEvent {
    match account_exists().await {
        Ok(true) => ConnectEvent::SetupAuthorization,
        _ => {
            window.set_always_on_top(true).unwrap();
            window.center().unwrap();
            window.show().unwrap();
            ConnectEvent::CreateAccount
        }
    }
}

/// Sends the mnemonic to the UI for the user to memorize.
#[tauri::command]
async fn get_mnemonic(password: String) -> Option<String> {
    Some(create_account(password).await.ok()?.into_phrase())
}

/// Ends the first round of communication between the UI and the signer.
#[tauri::command]
async fn end_connect(state: State<'_, PasswordStore>, window: Window) -> Result<(), ()> {
    window.hide().unwrap();
    state.clear().await;
    Ok(())
}

/// Runs the main Tauri application.
fn main() {
    const WALLET_FRONTEND_URL: &str = "http://localhost:8181";

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
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let password_store = app.state::<PasswordStore>().handle();
            spawn(async move {
                Service::build(window, password_store.setup().await)
                    .serve(WALLET_FRONTEND_URL)
                    .await
                    .unwrap();
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
