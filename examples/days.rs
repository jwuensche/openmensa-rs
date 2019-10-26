use openmensa_rs::{req_canteens, request::DayRequest};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    println!("Opening days from {}", list[0].name());
    let days = DayRequest::new(list[0].id())
        .with_start_date(chrono::Utc::today())
        .build()
        .await;
    println!("{:?}", days);
}
