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
