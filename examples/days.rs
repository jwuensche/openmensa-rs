use openmensa_rs::{req_canteens, req_opening_days};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    println!("Opening days from {}", list[0].name);
    let days = req_opening_days(list[0].clone()).await.unwrap();
    println!("{:?}", days);
}
