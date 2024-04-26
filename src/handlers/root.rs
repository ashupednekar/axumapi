use super::utils::invoke::call_python;

pub async fn handle() -> String {
    println!("root handler triggered...");

    // TODO: add openapi parsing... with module tag support

    call_python("music.artists", "list");
    "yay".to_owned()
}
