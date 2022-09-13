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

use core::time::Duration;
use manta_signer::{
    config::{Config, Setup},
    secret::{
        password_channel, Authorizer, Password, PasswordFuture, PasswordReceiver, PasswordSender,
        Secret, UnitFuture, SetupFuture, MnemonicSender, MnemonicReceiver, mnemonic_channel, UserSelection
    },
    serde::Serialize,
    service::Server,
    storage::Store,
    tokio::time::sleep,
    tokio::fs::{remove_file}
};
use tauri::{
    async_runtime::{spawn, JoinHandle}, CustomMenuItem, Manager, RunEvent, Runtime, State, SystemTray,
    SystemTrayEvent, SystemTrayMenu, Window, WindowEvent, AppHandle,
};

use manta_crypto::rand::OsRng;
use manta_pay::key::Mnemonic;

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
    pub fn new(window: Window, password_receiver: PasswordReceiver, mnemonic_receiver: MnemonicReceiver) -> Self {
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
            self.password_receiver.should_retry(true).await;
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
        self.password_receiver.should_retry(false).await;
    }
}

impl Authorizer for User {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move { self.request_password().await })
    }

    #[inline]
    fn setup<'s>(&'s mut self, data_exists : bool ) -> SetupFuture<'s> {
        let window = self.window.clone();
        Box::pin(async move {
            // NOTE: We have to wait here until the UI listener is registered.
            sleep(Duration::from_millis(500)).await;

            // creating a new mnemonic in case user will create a new account.
            let new_mnemonic = Mnemonic::sample(&mut OsRng);

            let payload = if data_exists {
                Setup::Login
            } else {
                // Mnemonic created here 
                Setup::CreateAccount(new_mnemonic)
            };

            window
                .emit("connect", payload.clone())
                .expect("The `connect` command failed to be emitted to the window.");
            
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

/// Mnemonic Store
pub type MnemonicStore = Store<MnemonicSender>;

/// App Handle Store
pub type AppHandleStore = Store<AppHandle>;

/// Abort Handle Store
pub type AbortHandleStore = Store<JoinHandle<()>>;

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


/// Sends the current `mnemonic` into storage from the UI.
#[tauri::command]
async fn send_mnemonic(mnemonic_store: State<'_,MnemonicStore>,mnemonic: String) -> Result<(),()> {

    // Mnemonic is assumed to be valid because it is validated by front end bip39 library.

    if let Some(store) = &mut *mnemonic_store.lock().await {

        let recovered_mnemonic = Mnemonic::new(mnemonic).expect("Unable to generate recovered Mnemonic.");
        store.load_exact(recovered_mnemonic).await;
    }
    Ok(())

}


/// Sets the user's selection of whether to create a new account or recover
/// using a seed phrase.
#[tauri::command]
async fn create_or_recover(mnemonic_store: State<'_, MnemonicStore>, selection:String) -> Result<(),()> {

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
    app_handle_store: State<'_,AppHandleStore>,
    abort_handle_store: State<'_,AbortHandleStore>,
    password_store: State<'_, PasswordStore>,
    mnemonic_store: State<'_, MnemonicStore>
) -> Result<(),()> {

    let config = Config::try_default().expect("Unable to generate the default server configuration.");
    remove_file(config.data_path.clone()).await.expect("File removal failed.");



    if let Some(handle) = &mut *abort_handle_store.lock().await {
        handle.abort();
    }

    let (password_sender, password_receiver) = password_channel();
    let (mnemonic_sender,mnemonic_receiver) = mnemonic_channel();
    password_store.set(password_sender).await;
    mnemonic_store.set(mnemonic_sender).await;

    let app_handle_guard = app_handle_store.lock().await;

    let app_handle = app_handle_guard.as_ref().unwrap();

    let new_window = app_handle.get_window("main").expect("Unable to open option");

    let new_handle = spawn(async move {

        let new_server = Server::build(config, User::new(new_window, password_receiver, mnemonic_receiver))
        .await
        .expect("Unable to build manta-signer");

        new_server.start().await.expect("Unable to start manta-signer");
    });


    abort_handle_store.set(new_handle).await;

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
                let (mnemonic_sender,mnemonic_receiver) = mnemonic_channel();
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
            reset_account
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
                "main" => app.exit(0),
                _ => unreachable!("There are no other windows."),
            }
        }
        _ => (),
    })
}
