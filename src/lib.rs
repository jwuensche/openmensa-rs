//! This crate provides a way to interact with the [openmensa api](https://openmensa.org/)
//! it serializes the request out of structures `CanteenRequest`, `DayRequest` and `MealRequest`.
//!
//! A simple example would be requesting a list of all available canteens in the api.
//!
//! ```rust
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use openmensa_rs::request::CanteenRequest;
//!
//! let list = CanteenRequest::new().build().await.unwrap();
//! # })
//! ```
//!
//! For a closer look on what you can specify in these requests go over to the documentation of these structs to see all available options and a more in-depth example.

use chrono::Date;
#[macro_use]

mod canteen;
mod day;
mod error;
mod meal;
mod price;
pub mod request;

pub use canteen::{Canteen, CoordinatePair};
pub use day::Day;
pub use error::RequestError;
pub use meal::Meal;
pub use price::Price;

const BASE_URL: &str = "https://openmensa.org/api/v2";

/// Short-hand to get all canteens.
pub async fn req_canteens() -> Result<Vec<Canteen>, RequestError> {
    let canteens: Vec<Canteen> = request::CanteenRequest::new().build().await?;
    Ok(canteens)
}

/// Short-hand to get the opening days starting from today from the given canteen.
pub async fn req_opening_days(canteen: Canteen) -> Result<Vec<Day>, RequestError> {
    let days: Vec<Day> = request::DayRequest::new(canteen.id()).build().await?;
    Ok(days)
}

/// Short-hand to get all meals offered on the specified day from one canteen.
pub async fn req_meals(
    canteen: Canteen,
    date: Date<chrono::Utc>,
) -> Result<Vec<Meal>, RequestError> {
    let meals: Vec<Meal> = request::MealRequest::new(canteen.id(), date)
        .build()
        .await?;
    Ok(meals)
}
