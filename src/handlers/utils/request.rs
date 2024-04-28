use axum::http::header::HeaderMap;
use pyo3::types::PyDict;

/*fn headers_to_dict(headers: HeaderMap) -> PyDict {
    let mut dict = PyDict::new();
    for (key, value) in headers.iter() {
        dict.set_item(key.as_str(), value.to_str().unwrap_or(""))
            .unwrap();
    }
    dict
}*/

fn parse_cookies(cookie_str: &str) -> std::collections::HashMap<String, String> {
    let mut cookie_map = std::collections::HashMap::new();

    for pair in cookie_str.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                cookie_map.insert(key.to_string(), value.to_string());
            }
        }
    }

    cookie_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_parser() {
        let cookie_str = String::from("tenant=ashu;book=one");
        let cookies = parse_cookies(&cookie_str);
        println!("cookies: {:?}", cookies);
    }
}
