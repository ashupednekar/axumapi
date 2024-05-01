use axum::extract::OriginalUri;
use std::fs::metadata;

pub async fn get_import_module(uri: &str) -> Option<String> {
    println!("uri: {}", uri);

    let mut module_path: String = String::from(uri);
    let mut import_str: String = String::from("");

    if !module_path.ends_with("/") {
        module_path.push_str("/")
    }

    module_path = module_path[1..module_path.len()].to_string();
    while !module_path.is_empty() {
        if let Ok(md) = metadata(&module_path) {
            if md.is_dir() {
                println!("Found module: {}", &module_path);
                import_str.push_str(&module_path);
                import_str.push_str(":__init__.py");
                break;
            }
        } else {
            let mut submodule_path = module_path.clone();
            submodule_path.push_str(".py");
            if let Ok(md1) = metadata(&submodule_path) {
                if md1.is_file() {
                    println!("Found submodule: {}", submodule_path);
                    import_str.push_str(&submodule_path);
                    let i = import_str.rfind("/").unwrap();
                    import_str = format!(
                        "{}/:{}",
                        &import_str[0..i],
                        &import_str[i + 1..import_str.len()]
                    );
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
        None
    } else {
        println!("import str: {}", import_str);
        Some(import_str)
    }
}
