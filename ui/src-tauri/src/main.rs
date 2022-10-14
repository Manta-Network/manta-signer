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

use core::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use manta_signer::{
    config::{Config, Setup},
    secret::{
        mnemonic_channel, password_channel, Authorizer, MnemonicReceiver, MnemonicSender, Password,
        PasswordFuture, PasswordReceiver, PasswordSender, Secret, SetupFuture, UnitFuture,
        UserSelection,
    },
    serde::Serialize,
    service::Server,
    storage::Store,
    tokio::fs::remove_file,
};
use std::time::Instant;
use tauri::{
    async_runtime::{spawn, JoinHandle},
    AppHandle, CustomMenuItem, Manager, RunEvent, Runtime, State, SystemTray, SystemTrayEvent,
    SystemTrayHandle, SystemTrayMenu, Window, WindowEvent,
};

use manta_crypto::rand::OsRng;
use manta_pay::key::Mnemonic;
use manta_accounting::wallet::signer::ReceivingKeyRequest;

/// App State
///
/// Keeps track of global state flags that we need for specific behaviors.
#[derive(Debug)]
pub struct AppState {
    /// UI is Connected
    pub ui_connected: AtomicBool,

    /// Currently Authorising
    pub authorizing: AtomicBool,
}

impl AppState {
    /// Builds a new [`AppState`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            ui_connected: AtomicBool::new(false),
            authorizing: AtomicBool::new(false),
        }
    }

    /// Returns the UI connection status.
    #[inline]
    pub fn get_ui_connected(&self) -> bool {
        self.ui_connected.load(Ordering::Relaxed)
    }

    /// Sets the UI connection status.
    #[inline]
    pub fn set_ui_connected(&self, ui_connected: bool) {
        self.ui_connected.store(ui_connected, Ordering::Relaxed)
    }

    /// Returns the authorizing status.
    #[inline]
    pub fn get_authorizing(&self) -> bool {
        self.authorizing.load(Ordering::Relaxed)
    }

    /// Sets the authorizing status.
    #[inline]
    pub fn set_authorizing(&self, auth: bool) {
        self.authorizing.store(auth, Ordering::Relaxed);
    }
}

/// Application State
pub static APP_STATE: AppState = AppState::new();

/// While with a timeout
/// Loop over body code block until specified time elapses and exits executing a given code block
/// Needs to be a macro to be able to break early in the main loop body code block
/// i.e. loop over waiting to connect and then break the loop, or timeout with Error message after
/// a specified time.
macro_rules! while_w_timeout{
    ($body:block, $timeout_d:expr, $failure:block) => {{
        let time_start = Instant::now();
        let timeout = Duration::from_millis($timeout_d);
        loop {
            $body
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

    /// Mnemonic Receiver
    mnemonic_receiver: MnemonicReceiver,

    /// Waiting Flag
    waiting: bool,
}

impl User {
    /// Builds a new [`User`] from `window` and `password_receiver`.
    #[inline]
    pub fn new(
        window: Window,
        password_receiver: PasswordReceiver,
        mnemonic_receiver: MnemonicReceiver,
    ) -> Self {
        Self {
            window,
            password_receiver,
            mnemonic_receiver,
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
            self.password_receiver.send_retry(true).await;
        }
        let password = self.password_receiver.password().await;
        self.waiting = password.is_known();
        password
    }

    /// Requests selection from user, either to create account or recover old account.
    #[inline]
    async fn request_selection(&mut self) -> UserSelection {
        let user_selection = self.mnemonic_receiver.selection().await;
        user_selection
    }

    /// Requests mnemonic from user
    #[inline]
    async fn request_mnemonic(&mut self) -> Mnemonic {
        let mnemonic = self.mnemonic_receiver.mnemonic().await;
        mnemonic
    }

    /// Sends validation message when password was correctly matched.
    #[inline]
    async fn validate_password(&mut self) {
        self.waiting = false;
        self.password_receiver.send_retry(false).await;
    }
}

impl Authorizer for User {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move { self.request_password().await })
    }

    #[inline]
    fn setup<'s>(&'s mut self, data_exists: bool) -> SetupFuture<'s> {
        let window = self.window.clone();
        Box::pin(async move {
            // creating a new mnemonic in case user will create a new account.
            let new_mnemonic = Mnemonic::sample(&mut OsRng);

            let payload = if data_exists {
                Setup::Login
            } else {
                // Mnemonic created here
                Setup::CreateAccount(new_mnemonic)
            };

            while_w_timeout!(
                {
                    if APP_STATE.get_ui_connected() {
                        break;
                    }
                    window
                        .emit("connect", payload.clone())
                        .expect("The `connect` command failed to be emitted to the window.");
                },
                5000,
                {
                    panic!("Connection attempt timedout!");
                }
            );

            if data_exists {
                Setup::Login
            } else {
                // We need to wait here until user decides to 1. recover using seed phrase or 2. create new account.

                let user_selection = self.request_selection().await;

                // if user decides to create a new account then we can continue to build the server here.
                if let UserSelection::Create = user_selection {
                    return payload;
                }

                // now we have to wait again until we get the user's seed phrase.

                let user_seed_phrase = self.request_mnemonic().await;

                Setup::CreateAccount(user_seed_phrase)
            }
        })
    }

    #[inline]
    fn wake<T>(&mut self, prompt: &T) -> UnitFuture
    where
        T: Serialize,
    {
        APP_STATE.set_authorizing(true);
        self.emit("authorize", prompt);
        Box::pin(async move {})
    }

    #[inline]
    fn sleep(&mut self) -> UnitFuture {
        APP_STATE.set_authorizing(false);
        Box::pin(async move { self.validate_password().await })
    }
}

/// Password Store
pub type PasswordStore = Store<PasswordSender>;

/// Server Store
pub type ServerStore = Store<Server<User>>;

/// Mnemonic Store
pub type MnemonicStore = Store<MnemonicSender>;

/// App Handle Store
pub type AppHandleStore = Store<AppHandle>;

/// Abort Handle Store
pub type AbortHandleStore = Store<JoinHandle<()>>;
/// Called from the UI after it recieves a `connect` event.
///
/// To ensure proper connection you should emit `connect` continuously until the
/// [`AppState::ui_connected`] flag is `true` then stop. This is the only way for now to ensure they
/// are synchronized. Tauri is working on a better way.
#[tauri::command]
fn ui_connected() {
    APP_STATE.set_ui_connected(true);
}

/// Called when user wants to cancel recovery or when user wants to proceed with recovery
/// in either case, server needs to restart and setup function needs to be called again to
/// emit new `connect` event with new payload.
#[tauri::command]
fn ui_disconnected() {
    APP_STATE.set_ui_connected(false);
}

/// Sends the current `password` into storage from the UI.
#[tauri::command]
async fn send_password(
    password_store: State<'_, PasswordStore>,
    app_handle_store: State<'_, AppHandleStore>,
    password: String,
) -> Result<bool, ()> {
    if let Some(store) = &mut *password_store.lock().await {
        let result = store.load(Secret::new(password)).await;

        if !result {
            let app_handle_guard = app_handle_store.lock().await;
            let app_handle = app_handle_guard.as_ref().unwrap();
            let tray_handle = app_handle.tray_handle();

            // if result == true, it means user has successfully signed in, so we can now add the tray
            // menu item to reset account. Which will emit "reset_account" to the front-end, and will trigger
            // the load of the delete page.

            set_tray_reset(tray_handle, true).await;
        }

        Ok(result)
    } else {
        Ok(false)
    }
}

/// Adds or removes the reset option and view secret phrase option on the menu tray
///  depending on the value of `reset`.
async fn set_tray_reset(tray_handle: SystemTrayHandle, reset: bool) {
    let new_menu: SystemTrayMenu = if reset {
        // add the reset option
        let menu = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("about", "About"))
            .add_item(CustomMenuItem::new("exit", "Quit"))
            .add_item(CustomMenuItem::new("reset", "Reset"))
            .add_item(CustomMenuItem::new(
                "view secret recovery phrase",
                "View Secret Recovery Phrase",
            ));
        menu
    } else {
        // remove it
        let menu = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("about", "About"))
            .add_item(CustomMenuItem::new("exit", "Quit"));
        menu
    };

    tray_handle
        .set_menu(new_menu)
        .expect("Unable to update tray menu");
}

/// Stops the server from prompting for the password.
#[tauri::command]
async fn stop_password_prompt(password_store: State<'_, PasswordStore>) -> Result<(), ()> {
    if let Some(store) = &mut *password_store.lock().await {
        store.clear().await;
    }
    Ok(())
}

/// Sends the current `mnemonic` into storage from the UI.
#[tauri::command]
async fn send_mnemonic(
    mnemonic_store: State<'_, MnemonicStore>,
    mnemonic: String,
) -> Result<(), ()> {
    // Mnemonic is assumed to be valid because it is validated by front end bip39 library.

    if let Some(store) = &mut *mnemonic_store.lock().await {
        let recovered_mnemonic =
            Mnemonic::new(mnemonic).expect("Unable to generate recovered Mnemonic.");
        store.load_exact(recovered_mnemonic).await;
    }
    Ok(())
}

/// Sets the user's selection of whether to create a new account or recover
/// using a seed phrase.
#[tauri::command]
async fn create_or_recover(
    mnemonic_store: State<'_, MnemonicStore>,
    selection: String,
) -> Result<(), ()> {
    let selected_option = if selection == "Create" {
        UserSelection::Create
    } else {
        UserSelection::Recover
    };

    if let Some(store) = &mut *mnemonic_store.lock().await {
        store.load_selection(selected_option).await;
    }

    Ok(())
}

/// Restarts the server in case of account reset. This feature can be used to implement the cancel button
/// once recovery has started.
#[tauri::command]
async fn reset_account(
    delete: bool,
    app_handle_store: State<'_, AppHandleStore>,
    abort_handle_store: State<'_, AbortHandleStore>,
    password_store: State<'_, PasswordStore>,
    mnemonic_store: State<'_, MnemonicStore>,
) -> Result<(), ()> {
    let config =
        Config::try_default().expect("Unable to generate the default server configuration.");

    // delete flag is present in case user wants to restart the setup process, but there is no storage files to delete.
    if delete {
        remove_file(config.data_path_dolphin.clone())
            .await
            .expect("Dolphin file removal failed.");
        remove_file(config.data_path_calamari.clone())
            .await
            .expect("Calamari file removal failed.");
        remove_file(config.data_path_manta.clone())
            .await
            .expect("Manta file removal failed.");
    }

    if let Some(handle) = &mut *abort_handle_store.lock().await {
        handle.abort();
    }

    let (password_sender, password_receiver) = password_channel();
    let (mnemonic_sender, mnemonic_receiver) = mnemonic_channel();
    password_store.set(password_sender).await;
    mnemonic_store.set(mnemonic_sender).await;

    let app_handle_guard = app_handle_store.lock().await;
    let app_handle = app_handle_guard.as_ref().unwrap();
    let tray_handle = app_handle.tray_handle();
    let new_window = app_handle
        .get_window("main")
        .expect("Unable to open option");

    let server_store_clone = app_handle.state::<ServerStore>().inner().clone();

    let new_handle = spawn(async move {
        let new_server = Server::build(
            config,
            User::new(new_window, password_receiver, mnemonic_receiver),
        )
        .await
        .expect("Unable to build manta-signer");

        server_store_clone.set(new_server.clone()).await;

        new_server
            .start()
            .await
            .expect("Unable to start manta-signer");
    });

    // Removing the reset account menu tray item.

    set_tray_reset(tray_handle, false).await;

    abort_handle_store.set(new_handle).await;

    Ok(())
}

/// Returns receiving keys to front end to display once user is logged in.
#[tauri::command]
async fn receiving_keys(server_store: State<'_, ServerStore>) -> Result<Vec<String>, ()> {
    if let Some(store) = &mut *server_store.lock().await {
        let keys = store.get_receiving_keys(ReceivingKeyRequest::GetAll).await;
        return keys
    }
    Err(())
}

/// Exports the user recovery phrase upon successful password match.
#[tauri::command]
async fn get_recovery_phrase(
    prompt: String,
    server_store: State<'_, ServerStore>,
) -> Result<Mnemonic, ()> {
    if let Some(store) = &mut *server_store.lock().await {
        let mnemonic = store
            .get_stored_mnemonic(&prompt)
            .await
            .expect("Unable to fetch mnemonic");
        Ok(mnemonic)
    } else {
        Err(())
    }
}

/// Cancels the current signing transaction within the server, allowing for
/// new signing transactions to be sent.
#[tauri::command]
async fn cancel_sign(server_store: State<'_, ServerStore>) -> Result<(), ()> {
    if let Some(store) = &mut *server_store.lock().await {
        store.cancel_signing().await;
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
                    "reset" => {
                        window(app, "main").show().expect("Unable to show window");
                        window(app, "main")
                            .emit("tray_reset_account", {})
                            .expect("Unable to emit reset tray event to window.");
                    }
                    "view secret recovery phrase" => {
                        window(app, "main")
                            .emit("show_secret_phrase", {})
                            .expect("Unable to emit reset tray event to window.");
                    }
                    "exit" => app.exit(0),
                    _ => {}
                }
            }
        })
        .manage(PasswordStore::default())
        .manage(ServerStore::default())
        .manage(MnemonicStore::default())
        .manage(AppHandleStore::default())
        .manage(AbortHandleStore::default())
        .setup(|app| {
            let window = window(app, "main");
            let password_store = app.state::<PasswordStore>().inner().clone();
            let server_store = app.state::<ServerStore>().inner().clone();
            let mnemonic_store = app.state::<MnemonicStore>().inner().clone();
            let app_handle_store = app.state::<AppHandleStore>().inner().clone();
            let abort_handle = app.state::<AbortHandleStore>().inner().clone();
            let app_handle = app.handle().clone();

            let join_handle = spawn(async move {
                let (password_sender, password_receiver) = password_channel();
                let (mnemonic_sender, mnemonic_receiver) = mnemonic_channel();
                let user = User::new(window, password_receiver, mnemonic_receiver);
                password_store.set(password_sender).await;
                mnemonic_store.set(mnemonic_sender).await;
                app_handle_store.set(app_handle).await;
                let server = Server::build(config, user)
                    .await
                    .expect("Unable to build manta-signer server.");
                server_store.set(server.clone()).await;
                server
                    .start()
                    .await
                    .expect("Unable to build manta-signer service.");
            });

            spawn(async move {
                abort_handle.set(join_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_password,
            stop_password_prompt,
            create_or_recover,
            send_mnemonic,
            reset_account,
            ui_connected,
            ui_disconnected,
            receiving_keys,
            get_recovery_phrase,
            cancel_sign
        ])
        .build(tauri::generate_context!())
        .expect("Error while building UI.");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|app, event| match event {
        RunEvent::Ready => window(app, "about").hide().expect("Unable to hide window."),
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            api.prevent_close();
            match label.as_str() {
                "about" => window(app, "about").hide().expect("Unable to hide window."),
                "main" => {
                    if APP_STATE.get_authorizing() {
                        window(app, "main").hide().expect("Unable to hide window.");
                        window(app, "main")
                            .emit("abort_auth", "Aborting Authorization")
                            .expect("Failed to abort authorization");
                    } else {
                        app.exit(0);
                    }
                }
                _ => unreachable!("There are no other windows."),
            }
        }
        _ => (),
    })
}
