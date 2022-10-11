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

/// Queries signer server to get all receiving keys
pub async fn get_receiving_keys(service_url: &str) -> Result<Vec<String>, ()> {
    let client = reqwest::Client::new();
    let url = format!("http://{}{}", service_url, "/receivingKeys");
    let keys: Vec<String> = client
        .post(url)
        .json(&String::from("GetAll"))
        .send()
        .await
        .expect("Failed to get receiving keys")
        .json::<Vec<ReceivingKey>>()
        .await
        .expect("Failed to deserialize receiving keys")
        .into_iter()
        .map(|key| receiving_key_to_base58(&key))
        .collect();

    Ok(keys)
}
