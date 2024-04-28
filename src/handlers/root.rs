use super::utils::invoke::call_python;
use axum::{
    body::{Body, Bytes},
    extract::{Extension, Json, Path, Query, Request},
    http::{HeaderMap, Method},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;

pub async fn handle(
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: String,
) -> String {
    println!("root handler triggered...");
    println!("query params: {:?}", params);
    println!("method: {}", method);
    println!("headers: {:?}", headers);
    println!("body: {}", body);

    let r = call_python("examples/music:songs.py:list");
    r.unwrap().to_string()
}
