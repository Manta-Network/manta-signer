//! Updater Server Types

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewRelease {
    pub target: String,
    pub version: String,
    pub release_notes: String,
    pub url: String,
    pub signature: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdaterResponse {
    pub url: String,
    pub version: String,
    pub notes: String,
    pub signature: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct VersionInfo {
    pub url: String,
    pub version: String,
}
