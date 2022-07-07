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

use core::{fmt, marker::Unpin};
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

impl Level {
    /// Returns the loggging prefix for `self` as a static string.
    #[inline]
    const fn as_prefix(&self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Info => "INFO ",
            Self::Warn => "WARN ",
            Self::Error => "ERROR",
        }
    }
}

/// Prints the `display` as a log line to the `writer` with the given logging `level`.
#[inline]
pub async fn log<W, D>(writer: &mut W, level: Level, display: D) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
    D: fmt::Display,
{
    writer
        .write_all(
            format!(
                "{} {}: {}\n",
                level.as_prefix(),
                chrono::offset::Utc::now(),
                display
            )
            .as_bytes(),
        )
        .await
}

/// Logs a single log line to the default writer of the given `$level`.
macro_rules! log_macro {
    ($level:expr, $($expr:expr),*) => {{
        $crate::log::log(&mut $crate::log::stdout(), $level, format!($($expr),*)).await
    }}
}

pub(crate) use log_macro as log;

/// Logs some trace information to the default writer.
macro_rules! trace_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Trace, $($expr),*)
    }}
}

pub(crate) use trace_macro as trace;

/// Logs some basic information to the default writer.
macro_rules! info_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Info, $($expr),*)
    }}
}

pub(crate) use info_macro as info;

/// Logs a warning to the default writer.
macro_rules! warn_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Warn, $($expr),*)
    }}
}

pub(crate) use warn_macro as warn;

/// Logs an error to the default writer.
macro_rules! error_macro {
    ($($expr:expr),*) => {{
        $crate::log::log!($crate::log::Level::Error, $($expr),*)
    }}
}

pub(crate) use error_macro as error;
