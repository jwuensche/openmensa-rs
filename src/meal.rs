use getset::{CopyGetters, Getters};
use serde::Deserialize;

use crate::price::Price;

/// Representation of a single meal.
#[derive(Deserialize, CopyGetters, Getters, Debug, Clone)]
pub struct Meal {
    #[get_copy = "pub"]
    id: u32,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    notes: Vec<String>,
    #[get = "pub"]
    prices: Price,
}
