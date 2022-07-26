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

//! Queries to Manta Signer Server

use manta_pay::config::{receiving_key_to_base58, ReceivingKey};
use reqwest::Error;

/// Queries signer server to get all receiving keys.
#[inline]
pub async fn get_receiving_keys(service_url: &str) -> Result<Vec<String>, Error> {
    Ok(reqwest::Client::new()
        .post(format!("http://{}{}", service_url, "/receivingKeys"))
        .json(&"GetAll")
        .send()
        .await?
        .json::<Vec<ReceivingKey>>()
        .await?
        .into_iter()
        .map(|key| receiving_key_to_base58(&key))
        .collect())
}
