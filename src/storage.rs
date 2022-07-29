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

//! Manta Signer State Handling Utilities

use alloc::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

/// Underlying Storage Container
type Storage<T> = Arc<Mutex<Option<T>>>;

/// Storage Entry
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""), Debug, Default(bound = ""))]
pub struct Store<T>(Storage<T>);

impl<T> Store<T> {
    /// Returns the mutex guard for the optional storage item.
    #[inline]
    pub async fn lock(&self) -> MutexGuard<Option<T>> {
        self.0.lock().await
    }

    /// Writes `state` to the state stored in `self`.
    #[inline]
    pub async fn set(&self, state: T) {
        self.write(move |t| *t = Some(state)).await
    }

    /// Writes the contents of `self` using `f`.
    #[inline]
    pub async fn write<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&mut Option<T>) -> U,
    {
        f(&mut *self.lock().await)
    }

    /// Writes to the internal state of `self` with `f`, first unwrapping the optional state.
    ///
    /// # Panics
    ///
    /// This method panics if the internal state is the `None` variant.
    #[inline]
    pub async fn unwrapping_write<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        self.write(move |t| f(t.as_mut().expect("Internal state was missing.")))
            .await
    }

    /// Reads the contents of `self` and sends them to `f`.
    #[inline]
    pub async fn update<U, F>(&self, f: F) -> U
    where
        F: FnOnce(Option<&mut T>) -> U,
    {
        f(self.lock().await.as_mut())
    }

    /// Reads the contents of `self`, unwrapping the optional state and then sends it to `f`.
    ///
    /// # Panics
    ///
    /// This method panics if the internal state is the `None` variant.
    #[inline]
    pub async fn unwrapping_update<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        self.update(move |t| f(t.expect("Internal state was missing.")))
            .await
    }
}
