use axum::http::header::{HeaderMap, HeaderValue};

/*fn headers_to_dict(headers: HeaderMap) -> PyDict {
    let mut dict = PyDict::new();
    for (key, value) in headers.iter() {
        dict.set_item(key.as_str(), value.to_str().unwrap_or(""))
            .unwrap();
    }
    dict
}*/

pub fn parse_cookies(headers: &mut HeaderMap) -> std::collections::HashMap<String, String> {
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
}
