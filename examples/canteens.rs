use openmensa_rs::CanteenRequest;

#[tokio::main]
async fn main() {
    println!("Canteens known: ");
    let list = CanteenRequest::new().build().await;
    println!("{:?}", list);
}
