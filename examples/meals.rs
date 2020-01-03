use openmensa_rs::{req_canteens, request::MealRequest};

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
    println!(
        "Meals in the {} canteen on the {}",
        canteen.0,
        chrono::Utc::today()
    );
    let meals = MealRequest::new(canteen.1, chrono::Utc::today())
        .build()
        .await
        .unwrap();
    println!("{:#?}", meals);
}
