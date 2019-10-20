use crate::price::Price;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Meal {
    id: u32,
    name: String,
    notes: Vec<String>,
    prices: Price,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyMeals {
    date: String,
    closed: bool,
    meals: Vec<Meal>,
}
