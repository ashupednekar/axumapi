use super::utils::invoke::call_python;

pub async fn handle() -> String {
    println!("root handler triggered...");
    let path = "a.py".to_owned();
    let class = "A".to_owned();
    let method = "a".to_owned();
    call_python(path, class, method);
    "yay".to_owned()
}
