//! Update Server

use crate::types::NewRelease;
use endpoints::{get_latest, handle_rejection, healthcheck, new_release, release_info, updates};
use log::info;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::net::SocketAddr;
use warp::Filter;

mod auth;
mod endpoints;
mod types;

#[tokio::main]
async fn main() {
    println!("Hello world!");
    env_logger::init();
    dotenv::dotenv().expect("Dotenv init failed");
    let client = DynamoDbClient::new(Region::UsEast2);
    let listen_on: SocketAddr = std::env::var("BIND_ADDR")
        .expect("No BIND_ADDR in env!")
        .parse()
        .expect("Invalid BIND_ADDR!");
    let uc = client.clone();
    let updater = warp::get()
        .and(warp::path("updates"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(move |target: String, version: String| {
            let c = uc.clone();
            updates(target, version, c)
        });
    let uc = client.clone();
    let latests = warp::get()
        .and(warp::path("get_latest"))
        .and(warp::path::param())
        .and_then(move |target: String| {
            let c = uc.clone();
            get_latest(target, c)
        });
    let hc = warp::get()
        .and(warp::path("healthcheck"))
        .and_then(healthcheck);
    let notifier = warp::post()
        .and(auth::do_auth())
        .untuple_one()
        .and(warp::path("new_release"))
        .and(release_info())
        .and_then(move |release: NewRelease| {
            info!("New release registered: {release:?}");
            let c = client.clone();
            new_release(release, c)
        });
    info!("Server starting, will listen on {listen_on}");
    warp::serve(
        hc.or(latests)
            .or(updater)
            .or(notifier)
            .recover(handle_rejection),
    )
    .run(listen_on)
    .await
}
