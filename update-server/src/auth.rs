//! Update Server Auth

use crate::types::*;
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use log::error;
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};
use serde::Deserialize;
use sha2::Sha256;
use warp::{reject, Filter};

const UPDATER_KEY_ID: &str = "updater-service";
const TOKEN_START: &str = "Bearer ";

#[derive(Debug, Deserialize)]
struct Token {
    user: String,
    exp: i64,
}

pub fn do_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Copy {
    warp::header::<String>("Authorization").and_then(|value| async move {
        if is_valid(value).await {
            Ok(())
        } else {
            Err(reject::custom(Unauthorized))
        }
    })
}

async fn is_valid(value: String) -> bool {
    let client = SecretsManagerClient::new(Region::UsEast2);
    let mut request = GetSecretValueRequest::default();
    request.secret_id = UPDATER_KEY_ID.into();
    if !value.trim().starts_with(TOKEN_START) {
        error!("Improperly formed header {value}");
        return false;
    }
    let value = &value.trim()[TOKEN_START.len()..];
    match client.get_secret_value(request).await {
        Ok(response) => {
            let secret = response.secret_string.unwrap();
            let key: Hmac<Sha256> = match Hmac::new_from_slice(secret.as_bytes()) {
                Ok(k) => k,
                Err(e) => {
                    error!("Bad key! {e:?}");
                    return false;
                }
            };
            let Token { user, exp } = match value.verify_with_key(&key) {
                Ok(value) => value,
                Err(e) => {
                    error!("Token verify failed {e:?}");
                    return false;
                }
            };
            let now = Utc::now().timestamp();
            user == "updates" && exp > now
        }
        Err(e) => {
            error!("Key fetch error {e:?}");
            false
        }
    }
}
