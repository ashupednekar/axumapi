use super::utils::invoke::call_python;
use super::utils::request::parse_cookies;
use axum::{
    extract::{OriginalUri, Query},
    http::{HeaderMap, Method},
};
use std::collections::HashMap;
use std::fs::metadata;
use std::path::Path;

/*let pl = uri.split("/");
let vec = pl.collect::<Vec<&str>>();
for p in vec.iter().rev() {
    if p.len() > 0 {
        module_path.push_str(p);
        println!("checking path: {}", module_path);
        let md = metadata(module_path.clone()).unwrap();
        if md.is_dir() {
            import_str.push_str(&module_path.clone());
            import_str.push_str(":__init__.py")
        } else if md.is_file() {
            import_str.push_str(&module_path.clone());
        }
        module_path.push_str("/");
    }
}*/

pub async fn get_import_str(uri: &OriginalUri) -> String {
    let mut uri = uri.to_string();
    let mut tmp = uri.split("?");
    uri = tmp.next().unwrap().to_string();

    println!("uri: {}", uri);

    let mut module_path: String = uri.clone();
    let mut import_str: String = String::from("");

    if !module_path.ends_with("/") {
        module_path.push_str("/")
    }

    module_path = module_path[1..module_path.len()].to_string();
    while !module_path.is_empty() {
        if let Ok(md) = metadata(&module_path) {
            if md.is_dir() {
                println!("Found root module: {}", &module_path);
                import_str.push_str(&module_path);
                import_str.push_str(":__init__.py");
                break;
            }
        } else {
            let mut submodule_path = module_path.clone();
            submodule_path.push_str(".py");
            if let Ok(md1) = metadata(&submodule_path) {
                if md1.is_file() {
                    println!("Found module: {}", submodule_path);
                    import_str.push_str(&submodule_path);
                    break;
                }
            }
        }

        if let Some(last_slash) = module_path.rfind('/') {
            module_path.truncate(last_slash);
        } else {
            module_path.clear();
        }
    }

    if module_path.is_empty() {
        println!("Not found");
    }
    println!("import str: {}", import_str);
    import_str
}

pub async fn handle(
    uri: OriginalUri,
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: String,
) -> String {
    /*println!("root handler triggered...");
    println!("query params: {:?}", params);
    println!("method: {}", method);
    println!("headers: {:?}", headers);
    println!("body: {}", body);
    let cookies = parse_cookies(&headers);
    println!("cookies: {:?}", cookies)
    */

    let import_str = get_import_str(&uri).await;
    let r = call_python(
        "examples/spotify/albums/:__init__.py:list",
        headers,
        params,
        body,
    );
    r.unwrap().to_string()
}
