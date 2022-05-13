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

//! Logging Utilities

#![allow(unused_imports, unused_macros)] // NOTE: We are exposing them as a library for this crate.

use core::fmt;
use core::marker::Unpin;
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

pub use tokio::io::stdout;

/// Log Level
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Level {
    /// Trace
    Trace,

    /// Information
    Info,

    /// Warning
    Warn,

    /// Error
    Error,
}

impl fmt::Display for Level {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

///
#[inline]
pub async fn log<W, D>(writer: &mut W, level: Level, display: D) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
    D: fmt::Display,
{
    writer
        .write_all(format!("{} [{}]: {}\n", level, chrono::offset::Utc::now(), display).as_bytes())
        .await
}

///
macro_rules! log_macro {
    ($level:expr, $($expr:expr),*) => {{
        $crate::log::log(&mut $crate::log::stdout(), $level, format!($($expr),*)).await
    }}
}

pub(crate) use log_macro as log;

///
macro_rules! trace_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Trace, $($expr),*)
    }}
}

pub(crate) use trace_macro as trace;

///
macro_rules! info_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Info, $($expr),*)
    }}
}

pub(crate) use info_macro as info;

///
macro_rules! warn_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Warn, $($expr),*)
    }}
}

pub(crate) use warn_macro as warn;

///
macro_rules! error_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Error, $($expr),*)
    }}
}

pub(crate) use error_macro as error;
