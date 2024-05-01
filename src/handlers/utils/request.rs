use axum::extract::OriginalUri;
use axum::http::header::{HeaderMap, HeaderValue};
use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
/*fn headers_to_dict(headers: HeaderMap) -> PyDict {
    let mut dict = PyDict::new();
    for (key, value) in headers.iter() {
        dict.set_item(key.as_str(), value.to_str().unwrap_or(""))
            .unwrap();
    }
    dict
}*/

pub fn get_uri_str(uri: OriginalUri) -> String {
    let mut uri = uri.to_string();
    let mut tmp = uri.split("?");
    uri = tmp.next().unwrap().to_string();
    uri
}

pub fn extract_path_params(url: &str) -> (Vec<i32>, String) {
    let re = Regex::new(r"/(\d+)/?").unwrap();
    let mut params = Vec::new();
    let mut new_url = String::from(url);
    for capture in re.captures_iter(url) {
        let param = capture.get(1).unwrap().as_str().to_string();
        let p: i32 = param.parse().expect("not an int");
        params.push(p);
        new_url = new_url.replace(&param, "");
    }
    new_url = new_url.replace("//", "/");
    new_url = new_url
        .replace("https:/", "https://")
        .replace("http:/", "http://");

    (params, new_url)
}

pub fn parse_cookies(headers: &HeaderMap) -> std::collections::HashMap<String, String> {
    let mut cookie_map = std::collections::HashMap::new();
    let cookie_str = headers
        .get("cookie")
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or("");
    println!("cookie str: {:?}", cookie_str);
    for pair in cookie_str.split(';') {
        let mut parts = pair.trim().split('=');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                cookie_map.insert(key.to_string(), value.to_string());
            }
        }
    }
    cookie_map
}

pub fn get_payload_map(body: &str) -> HashMap<String, Vec<String>> {
    let payload = match body.is_empty() {
        true => Ok(HashMap::new()),
        false => serde_json::from_str(&body),
    };
    payload.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_parser_cookie_present() {
        let mut headers = HeaderMap::new();
        headers.append(
            "cookie",
            HeaderValue::from_str("tenant=ashu;book=one").unwrap(),
        );
        let cookies = parse_cookies(&mut headers);
        println!("cookies: {:?}", cookies);
    }

    #[test]
    fn test_cookie_parser_cookie_absent() {
        let mut headers = HeaderMap::new();
        let cookies = parse_cookies(&mut headers);
        println!("cookies: {:?}", cookies);
    }

    #[test]
    fn test_extract_path_params_single_end() {
        let url = String::from("http://localhost:3000/examples/spotify/albums/user/1/");
        let (params, new_url) = extract_path_params(&url);
        assert_eq!(new_url, url.replace("/1", ""));
        assert_eq!(params, vec![1])
    }

    #[test]
    fn test_extract_path_params_single_middle() {
        let url = String::from("http://localhost:3000/examples/spotify/albums/1/user/");
        let (params, new_url) = extract_path_params(&url);
        assert_eq!(new_url, url.replace("/1", ""));
        assert_eq!(params, vec![1])
    }

    #[test]
    fn test_extract_path_params_multiple() {
        let url = String::from("http://localhost:3000/examples/spotify/albums/1/user/1/");
        let (params, new_url) = extract_path_params(&url);
        assert_eq!(new_url, url.replace("/1", ""));
        assert_eq!(params, vec![1, 1])
    }
}
