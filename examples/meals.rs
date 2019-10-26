use openmensa_rs::{req_canteens, request::MealRequest};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    println!(
        "Meals in the {} canteen on the {}",
        list[0].name(),
        chrono::Utc::today()
    );
    let meals = MealRequest::new(list[0].id(), chrono::Utc::today())
        .build()
        .await
        .unwrap();
    println!("{:?}", meals);
}
