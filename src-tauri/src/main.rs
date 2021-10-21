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

use async_fs::{self as fs, File};
use bip0039::{Count, Mnemonic};
use codec::{Decode, Encode};
use manta_crypto::MantaSerDes;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::{io, sync::Arc};
use tauri::{
    async_runtime::{channel, RwLock, Sender},
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
};

/// Checks if a root seed exists at the canonical file path.
#[inline]
async fn account_exists() -> io::Result<bool> {
    Ok(fs::metadata("root_seed.aes").await?.is_file())
}

/// Creates a new account by building and saving a new root seed from the given `password`.
#[inline]
async fn create_account(password: String) -> Option<Mnemonic> {
    let mnemonic = Mnemonic::generate(Count::Words12);
    manta_api::save_root_seed(mnemonic.to_seed(""), password).ok()?;
    Some(mnemonic)
}

/// Transaction Type
#[derive(Deserialize, Serialize)]
enum TransactionType {
    PrivateTransfer { recipient: String },
    Reclaim,
}

/// Transaction Summary
#[derive(Deserialize, Serialize)]
struct TransactionSummary {
    transaction: TransactionType,
    amount: String,
    currency_symbol: String,
}

/// Loads a transaction request from the wallet frontend.
#[inline]
async fn load_request() -> Option<TransactionSummary> {
    // TODO: Load request from wallet frontend.
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Some(TransactionSummary {
        transaction: TransactionType::Reclaim,
        amount: String::from(""),
        currency_symbol: String::from(""),
    })
}

/// Sends an authorized proof payload to the wallet frontend.
#[inline]
async fn send_authorized(payload: ()) {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}

/// Sends a decline notice to the wallet fronend.
#[inline]
async fn send_declined() {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}

/// Generates ZKP proof payload to send to wallet frontend.
#[inline]
fn generate_proof_payload(password: String) -> () {
    std::thread::sleep(std::time::Duration::from_secs(10))
}

/// Password State Type
type PasswordStateType = Arc<RwLock<Option<Sender<Option<String>>>>>;

/// Password State
#[derive(Default)]
struct Password(PasswordStateType);

impl Password {
    /// Builds a new state from an empty `state` and `sender`.
    #[inline]
    async fn setup(state: PasswordStateType, sender: Sender<Option<String>>) {
        *state.write().await = Some(sender);
    }

    /// Loads a new password into the state.
    #[inline]
    async fn load(&self, password: String) {
        if let Some(state) = &*self.0.read().await {
            state.send(Some(password)).await.unwrap();
        }
    }

    /// Clears the password state.
    #[inline]
    async fn clear(&self) {
        if let Some(state) = &*self.0.read().await {
            state.send(None).await.unwrap();
        }
    }
}

/// Loads the current `password` into storage from the UI.
#[tauri::command]
async fn load_password(state: State<'_, Password>, password: String) -> Result<(), ()> {
    state.load(password).await;
    Ok(())
}

/// Removes the current password from storage.
#[tauri::command]
async fn clear_password(state: State<'_, Password>) -> Result<(), ()> {
    state.clear().await;
    Ok(())
}

/// Connection Event
#[derive(Deserialize, Serialize)]
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
            show(&window);
            ConnectEvent::CreateAccount
        }
    }
}

/// Sends the mnemonic to the UI for the user to memorize.
#[tauri::command]
async fn get_mnemonic(password: String) -> String {
    create_account(password)
        .await
        .expect("Unable to create account.")
        .into_phrase()
}

/// Ends the first round of communication between the UI and the signer.
#[tauri::command]
async fn end_connect(state: State<'_, Password>, window: Window) -> Result<(), ()> {
    hide(&window);
    state.clear().await;
    Ok(())
}

/// Shows the given `window`.
#[inline]
fn show(window: &Window) {
    window.set_always_on_top(true).unwrap();
    window.center().unwrap();
    window.show().unwrap();
}

/// Hides the given `window`.
#[inline]
fn hide(window: &Window) {
    window.hide().unwrap()
}

/// Runs the main Tauri application.
fn main() {
    let mut app = tauri::Builder::default()
        .system_tray(
            SystemTray::new()
                .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("exit", "Quit"))),
        )
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } if id == "exit" => app.exit(0),
            _ => {}
        })
        .manage(Password::default())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let state = app.state::<Password>().0.clone();
            tauri::async_runtime::spawn(async move {
                let (password_sender, mut password_receiver) = channel(1);
                Password::setup(state, password_sender).await;
                password_receiver.recv().await;
                loop {
                    match load_request().await {
                        Some(summary) => window.emit("authorize", summary).unwrap(),
                        _ => continue,
                    }
                    show(&window);
                    if let Some(password) = password_receiver.recv().await {
                        match password {
                            Some(password) => {
                                send_authorized(generate_proof_payload(password)).await;
                                hide(&window);
                            }
                            _ => {
                                hide(&window);
                                send_declined().await
                            }
                        }
                    }
                }
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
        .expect("Error while building tauri application.");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(move |_, _| {})
}
