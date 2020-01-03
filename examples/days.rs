use openmensa_rs::{req_canteens, request::DayRequest};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    let canteen = list
        .iter()
        .filter_map(|elem| {
            if elem.name().contains("Hannover") {
                Some((elem.name(), elem.id()))
            } else {
                None
            }
        })
        .nth(0)
        .unwrap();
    println!("Opening days from {}", canteen.0);
    let days = DayRequest::new(canteen.1)
        .with_start_date(chrono::Utc::today())
        .build()
        .await;
    println!("{:#?}", days);
}
