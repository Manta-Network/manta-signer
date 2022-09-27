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

//! Test Signer Server

use manta_crypto::rand::{CryptoRng, OsRng, RngCore, Sample};
use manta_pay::key::Mnemonic;
use manta_signer::{
    config::{Config, Setup},
    secret::{Authorizer, Password, PasswordFuture, SecretString, SetupFuture},
    service::{Error, Server},
};

/// Mock User
pub struct MockUser {
    /// User Password
    password: SecretString,
}

impl MockUser {
    /// Builds a new [`MockUser`] from `password`.
    #[inline]
    pub fn new<R>(rng: &mut R) -> Self
    where
        R: CryptoRng + RngCore + ?Sized,
    {
        Self {
            password: SecretString::new(u128::gen(rng).to_string()),
        }
    }
}

impl Authorizer for MockUser {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move { Password::from_known(self.password.clone()) })
    }

    #[inline]
    fn setup(&mut self, data_exists: bool) -> SetupFuture {
        let new_mnemonic = Mnemonic::sample(&mut OsRng);
        if data_exists {
            Box::pin(async move { Setup::Login })
        } else {
            Box::pin(async move { Setup::CreateAccount(new_mnemonic) })
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let test_dir = tempfile::tempdir()?;
    let mut config =
        Config::try_default().expect("Unable to generate the default server configuration.");
    config.data_path = test_dir.path().join("storage.dat");
    if let Some(url) = std::env::args().nth(1) {
        config.service_url = url;
    }
    Server::build(config, MockUser::new(&mut OsRng))
        .await?
        .start()
        .await
}
