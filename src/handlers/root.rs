use super::utils::invoke::call_python;
use super::utils::request::parse_cookies;
use axum::{
    extract::{Extension, Json, Path, Query, Request},
    http::{HeaderMap, Method},
};
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
    let cookies = parse_cookies(&headers);
    println!("cookies: {:?}", cookies);
    let r = call_python("examples/music:songs.py:list", headers, params, body);
    r.unwrap().to_string()
}
