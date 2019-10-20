use openmensa_rs::{req_canteens, req_meals};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    println!("Opening days from {}", list[0].name);
    let meals = req_meals(list[0].clone(), chrono::Utc::now().date()).await.unwrap();
    println!("{:?}", meals);
}
