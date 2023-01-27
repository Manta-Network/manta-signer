//! Updater Server Endpoints

use crate::types::*;
use log::{error, info, warn};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use std::collections::HashMap;
use warp::{http::StatusCode, reject, reply::with_status, Filter, Reply};

const MAX_JSON_BODY: u64 = 1024 * 16;
const RELEASES_DYNAMODB: &str = "release-versions";

fn customize_target(target: String, version: &String) -> String {
    let part = if target.starts_with("win") {
        version.split('.').next_back().unwrap()
    } else {
        version.split('-').next_back().unwrap()
    };
    format!("{target}:{part}")
}

pub fn release_info() -> impl Filter<Extract = (NewRelease,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_JSON_BODY).and(warp::body::json())
}

fn make_response(my_version: String, item: HashMap<String, AttributeValue>) -> impl warp::Reply {
    let item = match item.get("item") {
        Some(i) if i.m.is_some() => i.m.as_ref().unwrap(),
        _ => {
            return with_status("No Content", StatusCode::NO_CONTENT).into_response();
        }
    };

    if ["url", "release_notes", "version", "signature"]
        .into_iter()
        .all(|k| item.contains_key(k))
    {
        let url = item.get("url").unwrap().s.as_ref().unwrap().into();
        let version = item.get("version").unwrap().s.as_ref().unwrap().into();
        let notes = item
            .get("release_notes")
            .unwrap()
            .s
            .as_ref()
            .unwrap()
            .into();
        let signature = item.get("signature").unwrap().s.as_ref().unwrap().into();

        let resp = UpdaterResponse {
            url,
            version,
            notes,
            signature,
        };

        info!("Response {resp:?}");
        if my_version != resp.version {
            warp::reply::json(&resp).into_response()
        } else {
            with_status("No Content", StatusCode::NO_CONTENT).into_response()
        }
    } else {
        error!("DB entry did not contain all the required keys");
        with_status("No Content", StatusCode::NO_CONTENT).into_response()
    }
}

async fn update_available(
    target: String,
    version: String,
    client: DynamoDbClient,
) -> Option<impl warp::Reply> {
    let mut get_item_input = GetItemInput::default();
    get_item_input.table_name = RELEASES_DYNAMODB.into();

    let mut key = AttributeValue::default();
    let target = customize_target(target, &version);
    info!("UPDATE query {target}");
    key.s = Some(target);
    get_item_input.key = HashMap::from_iter([("target".to_string(), key)]);

    match client.get_item(get_item_input).await {
        Ok(output) => match output.item {
            Some(item) => Some(make_response(version, item)),
            None => None,
        },
        Err(e) => {
            error!("Server error: {e}");
            None
        }
    }
}

fn string_attribute(s: String) -> AttributeValue {
    let mut attr = AttributeValue::default();
    attr.s = Some(s);
    attr
}

fn map_attribute(m: HashMap<String, AttributeValue>) -> AttributeValue {
    let mut attr = AttributeValue::default();
    attr.m = Some(m);
    attr
}

pub async fn new_release(
    info: NewRelease,
    client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    let NewRelease {
        target,
        version,
        release_notes,
        url,
        signature,
    } = info;
    info!("New release {version}");

    let mut item = HashMap::new();
    item.insert("version".to_string(), string_attribute(version.clone()));
    item.insert("release_notes".to_string(), string_attribute(release_notes));
    item.insert("url".to_string(), string_attribute(url));
    item.insert("signature".to_string(), string_attribute(signature));

    let mut entry = HashMap::new();
    entry.insert("item".to_string(), map_attribute(item));
    let target = customize_target(target, &version);
    info!("NEW RELEASE target {target}");
    entry.insert("target".to_string(), string_attribute(target));

    let mut item = PutItemInput::default();
    item.table_name = RELEASES_DYNAMODB.into();
    item.item = entry;

    match client.put_item(item).await {
        Ok(_) => info!("Release made"),
        Err(e) => {
            error!("Bad stuff happened {e:?}");
            return Ok(
                with_status("Server Error", StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            );
        }
    }
    Ok(with_status("Good", StatusCode::OK).into_response())
}

pub async fn healthcheck() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(with_status("OK", StatusCode::OK).into_response())
}

async fn latests(target: String, client: DynamoDbClient) -> Option<impl warp::Reply> {
    let mut get_item_input = GetItemInput::default();
    get_item_input.table_name = RELEASES_DYNAMODB.into();
    let mut key = AttributeValue::default();
    info!("LATEST query {target}");
    key.s = Some(target.clone());
    get_item_input.key = HashMap::from_iter([("target".to_string(), key)]);
    match client.get_item(get_item_input).await {
        Ok(output) => match output.item {
            Some(item) => {
                let mut results = Vec::new();
                for ver in item.get("versions").unwrap().l.as_ref().unwrap() {
                    let ver = ver.s.as_ref().unwrap();
                    let mut get_item_input = GetItemInput::default();
                    get_item_input.table_name = RELEASES_DYNAMODB.into();
                    let mut key = AttributeValue::default();
                    key.s = Some(format!("{}:{}", target.clone(), ver));
                    get_item_input.key = HashMap::from_iter([("target".to_string(), key)]);
                    match client.get_item(get_item_input).await {
                        Ok(output) => match output.item {
                            Some(obj) => {
                                let item = obj.get("item").unwrap().m.as_ref().unwrap();
                                let url: String =
                                    item.get("url").unwrap().s.as_ref().unwrap().into();
                                let url = if url.ends_with(".zip") {
                                    url[..url.len() - ".zip".len()].to_string()
                                } else if url.ends_with(".app.tar.gz") {
                                    let url = url[..url.len() - ".app.tar.gz".len()].to_string();
                                    format!("{}_x64.dmg", url)
                                } else if url.ends_with(".tar.gz") {
                                    url[..url.len() - ".tar.gz".len()].to_string()
                                } else {
                                    url
                                };
                                let version =
                                    item.get("version").unwrap().s.as_ref().unwrap().into();
                                results.push(VersionInfo { url, version });
                            }
                            None => {
                                warn!("Version {} was not found for {}!", ver, target);
                            }
                        },
                        Err(e) => {
                            error!("Server error: {e}");
                            return None;
                        }
                    }
                }
                Some(warp::reply::json(&results).into_response())
            }
            None => None,
        },
        Err(e) => {
            error!("Server error: {e}");
            None
        }
    }
}

pub async fn get_latest(
    target: String,
    client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Request for {target}:latest");
    if let Some(releases) = latests(target, client).await {
        Ok(releases.into_response())
    } else {
        Ok(with_status("No Content", StatusCode::NO_CONTENT).into_response())
    }
}

pub async fn updates(
    target: String,
    version: String,
    client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Request for {target}:{version}");
    if let Some(release) = update_available(target, version, client).await {
        Ok(release.into_response())
    } else {
        Ok(with_status("No Content", StatusCode::NO_CONTENT).into_response())
    }
}

pub async fn handle_rejection(
    err: reject::Rejection,
) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(with_status("Not Found", StatusCode::NOT_FOUND).into_response())
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(with_status("Bad Request", StatusCode::BAD_REQUEST).into_response())
    } else if let Some(_) = err.find::<Unauthorized>() {
        Ok(with_status("Unauthorized", StatusCode::UNAUTHORIZED).into_response())
    } else {
        error!("Unhandled error type {err:?}");
        Ok(with_status("Error", StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }
}
