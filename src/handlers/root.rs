use super::utils::invoke::call_python;
use super::utils::request::{extract_path_params, get_uri_str, parse_cookies};
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

    let res = get_import_module(&url).await;
    match res {
        Some(m) => {
            println!("m: {}", m);

            let r = call_python(
                &format!("{}:list", m),
                query_params,
                path_params,
                headers,
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
