//! # Request Builders
//! This module contains the request builders, each have their own additional attributes which can be set to filter certain results from the response.
//!
//! # Example
//!
//! Each of the three request can carry additional information you may want to filter by e.g.
//! ```rust
//! use openmensa_rs::request::CanteenRequest;
//! use openmensa_rs::CoordinatePair;
//!
//! #[tokio::main]
//! async fn main() {
//!     let near_canteens = CanteenRequest::new()
//!         .with_near_coordinates(
//!             CoordinatePair::new(
//!                 52.1396188273019,
//!                 11.6475999355316,
//!             )
//!         )
//!         .build()
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! specifiy a location for canteens (for more option have a look at the structs themselves).
//!
//! Or the range from which opening days may be returned:
//! ```rust
//! use openmensa_rs::request::DayRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let near_canteens = DayRequest::new(1)
//!         .with_start_date(chrono::Utc::today())
//!         .build()
//!         .await
//!         .unwrap();
//! }
//! ```

mod canteen_request;
mod day_request;
mod meal_request;

pub use canteen_request::*;
pub use day_request::*;
pub use meal_request::*;
