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

//! Test Signer Server

use async_std::io;
use manta_signer::{
    config::Config,
    secret::{create_account, Authorization, Authorizer, Password},
    service::Service,
};
use rand::{
    distributions::{DistString, Standard},
    thread_rng, Rng,
};
use serde::Serialize;

/// Mock User
pub struct MockUser {
    /// Stored Password
    password: String,
}

impl MockUser {
    /// Builds a new [`MockUser`] from `password`.
    #[inline]
    fn new(password: String) -> Self {
        Self { password }
    }
}

impl Authorizer for MockUser {
    #[inline]
    fn authorize<T>(&mut self, prompt: T) -> Authorization
    where
        T: Serialize,
    {
        let _ = prompt;
        Box::pin(async move { Some(Password::Known(self.password.clone())) })
    }
}

/// Test Service
pub struct TestService(Service<MockUser>);

impl TestService {
    /// Builds a new [`TestService`] with the given `config` and a random password.
    #[inline]
    pub fn build(config: Config) -> Self {
        let mut rng = thread_rng();
        let length = rng.gen_range(20..50);
        Self(Service::build(
            config,
            MockUser::new(Standard.sample_string(&mut rng, length)),
        ))
    }

    /// Starts the test service on `listener`.
    #[inline]
    pub async fn serve(self) -> io::Result<()> {
        {
            create_account(
                self.0.config().await.root_seed_file,
                self.0.state().0.lock().await.authorizer.password.clone(),
            )
            .await
            .expect("Unable to create account for TestService.");
        }
        self.0.serve().await
    }
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let test_dir = tempfile::tempdir()?;
    let mut config =
        Config::try_default().expect("Unable to generate the default server configuration.");
    config.root_seed_file = test_dir.path().join("root_seed.aes");
    if let Some(url) = std::env::args().nth(1) {
        config.service_url = url;
    }
    TestService::build(config).serve().await
}
