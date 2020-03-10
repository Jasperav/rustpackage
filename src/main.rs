use crate::api::MessageMinimal;

mod api;

fn main() {
    println!("Hello, world!");

    let my_message = MessageMinimal::default();
    serde_json::to_string(&my_message).unwrap();
}

