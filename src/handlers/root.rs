use super::utils::invoke::call_python;

pub async fn handle() -> String {
    println!("root handler triggered...");

    // TODO: add openapi parsing... with module tag support

    let r = call_python("examples/music:songs.py:list");
    r.unwrap().to_string()
}
