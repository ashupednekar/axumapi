use super::utils::invoke::call_python;
use super::utils::request::parse_cookies;
use super::utils::router::get_import_module;
use axum::{
    extract::{OriginalUri, Query},
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn handle(
    uri: OriginalUri,
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: String,
) -> impl IntoResponse {
    println!("root handler triggered...");
    println!("query params: {:?}", params);
    println!("method: {}", method);
    println!("headers: {:?}", headers);
    println!("body: {}", body);
    let cookies = parse_cookies(&headers);
    println!("cookies: {:?}", cookies);

    let res = get_import_module(&uri).await;
    match res {
        Some(m) => {
            println!("m: {}", m);

            let r = call_python(
                "examples/spotify/albums/:__init__.py:list",
                headers,
                params,
                body,
            );
            let res = r.unwrap().to_string();
            (StatusCode::OK, res)
        }
        None => (
            StatusCode::NOT_FOUND,
            "api not found in current package".to_string(),
        ),
    }
}
