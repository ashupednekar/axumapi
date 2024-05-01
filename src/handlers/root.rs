use super::utils::invoke::call_python;
use super::utils::request::{extract_path_params, get_uri_str, parse_cookies};
use super::utils::router::{append_function_suffix, get_import_module};
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
    Query(query_params): Query<HashMap<String, String>>,
    body: String,
) -> impl IntoResponse {
    println!("root handler triggered...");
    println!("query params: {:?}", query_params);
    println!("method: {}", method);
    println!("headers: {:?}", headers);
    println!("body: {}", body);
    let cookies = parse_cookies(&headers);
    println!("cookies: {:?}", cookies);
    let (path_params, url) = extract_path_params(&get_uri_str(uri.clone()));

    let module = get_import_module(&url).await;
    match module {
        Some(m) => {
            println!("m: {}", m);
            let import_str = append_function_suffix(&m, method.as_str(), path_params.clone()).await;
            let r = call_python(&import_str, query_params, path_params, headers, body);
            match r {
                Ok(r) => (StatusCode::OK, r.to_string()),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            }
        }
        None => (
            StatusCode::NOT_FOUND,
            "api not found in current package".to_string(),
        ),
    }
}
