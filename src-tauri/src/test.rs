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

//! Manta Signer Testing Primitives

use crate::{secret::Password, service::Authorizer};
use futures::future::BoxFuture;
use serde::Serialize;

/// Mock User
pub struct MockUser {
    /// Stored Password
    password: String,
}

impl MockUser {
    /// Builds a new [`MockUser`] from `password`.
    #[inline]
    pub fn new(password: String) -> Self {
        Self { password }
    }
}

impl Authorizer for MockUser {
    #[inline]
    fn authorize<T>(&mut self, prompt: T) -> BoxFuture<'_, Option<Password>>
    where
        T: Serialize,
    {
        let _ = prompt;
        Box::pin(async move { Some(Password::Known(self.password.clone())) })
    }
}
