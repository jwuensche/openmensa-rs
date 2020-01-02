use openmensa_rs::{request::CanteenRequest, CoordinatePair};

#[tokio::main]
async fn main() {
    println!("Canteens known: ");
    let list = CanteenRequest::new()
        .with_near_coordinates(CoordinatePair::new(52.139_618, 11.647_599))
        .build()
        .await;
    println!("{:#?}", list);
}
