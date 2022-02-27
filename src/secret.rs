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

//! Signer Secrets

// TODO: Use password hashing abstractions from `manta-rs`.

use crate::config::Setup;
use futures::future::BoxFuture;
use manta_util::serde::Serialize;
use password_hash::{PasswordHashString, SaltString};

pub use password_hash::{Error as PasswordHashError, PasswordHasher, PasswordVerifier};
pub use secrecy::{ExposeSecret, Secret, SecretString};
pub use subtle::{Choice, ConstantTimeEq, CtOption};

/// Password Secret Wrapper
pub struct Password(CtOption<SecretString>);

impl Password {
    /// Builds a new [`Password`] from `password` if `is_known` evaluates to `true`.
    #[inline]
    pub fn new(password: SecretString, is_known: Choice) -> Self {
        Self(CtOption::new(password, is_known))
    }

    /// Builds a new [`Password`] from `password`.
    #[inline]
    pub fn from_known(password: SecretString) -> Self {
        Self::new(password, 1.into())
    }

    /// Builds a new [`Password`] with a no known value.
    #[inline]
    pub fn from_unknown() -> Self {
        Self::new(Secret::new(String::with_capacity(64)), 0.into())
    }

    /// Returns [`Some`] if `self` represents a known password.
    #[inline]
    pub fn known(self) -> Option<SecretString> {
        self.0.into()
    }

    /// Returns `true` if `self` represents a known password.
    #[inline]
    pub fn is_known(&self) -> bool {
        self.0.is_some().into()
    }
}

impl Default for Password {
    #[inline]
    fn default() -> Self {
        Self::from_unknown()
    }
}

/// Unit Future
///
/// This `type` is used by the [`setup`], [`wake`], and [`sleep`] methods of [`Authorizer`].
/// See their documentation for more.
///
/// [`setup`]: Authorizer::setup
/// [`wake`]: Authorizer::wake
/// [`sleep`]: Authorizer::sleep
pub type UnitFuture<'t> = BoxFuture<'t, ()>;

/// Password Future
///
/// This `type` is used by the [`password`](Authorizer::password) method of [`Authorizer`].
/// See its documentation for more.
pub type PasswordFuture<'t> = BoxFuture<'t, Password>;

/// Authorizer
pub trait Authorizer: 'static + Send {
    /// Retrieves the password from the authorizer.
    fn password(&mut self) -> PasswordFuture;

    /// Runs some setup for the authorizer using the `setup`.
    ///
    /// # Implementation Note
    ///
    /// For custom service implementations, this method should be called before any service is run.
    /// The [`service::start`] function already calls this method internally.
    ///
    /// [`service::start`]: crate::service::start
    fn setup<'s>(&'s mut self, setup: &'s Setup) -> UnitFuture<'s>;

    /// Prompts the authorizer with `prompt` so that they can be notified that their password is
    /// requested.
    ///
    /// # Implementation Note
    ///
    /// After [`wake`] is called, [`password`] should be called to retrieve the password. These are
    /// implemented as two separate methods so that [`password`] can be called multiple times for
    /// password retries. By default, [`wake`] does nothing.
    ///
    /// [`wake`]: Self::wake
    /// [`password`]: Self::password
    #[inline]
    fn wake<T>(&mut self, prompt: &T) -> UnitFuture
    where
        T: Serialize,
    {
        let _ = prompt;
        Box::pin(async move {})
    }

    /// Sends a message to the authorizer to end communication.
    ///
    /// # Implementation Note
    ///
    /// By default, [`sleep`](Self::sleep) does nothing.
    #[inline]
    fn sleep(&mut self) -> UnitFuture {
        Box::pin(async move {})
    }
}

/// Argon2 Hasher Type
pub type Argon2 = argon2::Argon2<'static>;

/// Password Hash
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordHash<H>
where
    H: PasswordHasher,
{
    /// Hash String
    hash: PasswordHashString,

    /// Hasher
    hasher: H,
}

impl<H> PasswordHash<H>
where
    H: PasswordHasher,
{
    /// Builds a new [`PasswordHash`] from `hasher` and `password`.
    #[inline]
    pub fn new(hasher: H, password: &[u8]) -> Self {
        // TODO: Use a randomized salt which is saved into the signer state.
        Self {
            hash: hasher
                .hash_password(
                    password,
                    &SaltString::b64_encode(b"default password salt")
                        .expect("Unable to construct password salt."),
                )
                .expect("Unable to hash password.")
                .serialize(),
            hasher,
        }
    }

    /// Builds a new [`PasswordHash`] from `password` using the default [`PasswordHasher`].
    #[inline]
    pub fn from_default(password: &[u8]) -> Self
    where
        H: Default,
    {
        Self::new(Default::default(), password)
    }

    /// Verifies that the hash of `password` matches the known password hash.
    #[inline]
    pub fn verify(&self, password: &[u8]) -> Result<(), PasswordHashError> {
        self.hasher
            .verify_password(password, &self.hash.password_hash())
    }

    /// Returns the hash output as a byte vector.
    #[inline]
    pub fn as_bytes(&self) -> Vec<u8> {
        self.hash
            .hash()
            .expect("This is guaranteed to contain the hash it was built with.")
            .as_bytes()
            .to_owned()
    }
}
