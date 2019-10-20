use openmensa_rs::{CanteenRequest};

#[tokio::main]
async fn main() {
    let list = CanteenRequest::new().build().await;
    println!("{:?}", list);
}
