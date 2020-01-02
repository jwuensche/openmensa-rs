use openmensa_rs::{request::CanteenRequest, CoordinatePair};

#[tokio::main]
async fn main() {
    println!("Canteens known: ");
    let list = CanteenRequest::new()
        .with_near_coordinates(CoordinatePair::new(52.139618827301902, 11.6475999355316))
        .build()
        .await;
    println!("{:#?}", list);
}
