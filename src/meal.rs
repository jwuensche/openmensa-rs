use crate::price::Price;
use chrono::{Date, Utc};
use http::Method;
use serde::Deserialize;
use surf::Request;

use crate::error::RequestError;
use crate::BASE_URL;

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

pub struct MealRequest {
    canteen_id: u8,
    date: Date<Utc>,
}

impl MealRequest {
    pub fn new(canteen_id: u8, date: Date<Utc>) -> Self {
        Self { canteen_id, date }
    }
    // pub fn with_meal_id(mut self, meal_id: u16) -> Self {
    //     self.meal_id = Some(meal_id);
    //     self
    // }
    pub async fn build(self) -> Result<Vec<Meal>, RequestError> {
        let list_json = Request::new(
            Method::GET,
            url::Url::parse(
                format!(
                    "{}/canteens/{}/days/{}/meals",
                    BASE_URL,
                    self.canteen_id,
                    self.date.format("%Y-%m-%d").to_string()
                )
                .as_str(),
            )?,
        )
        .recv_string()
        .await?;
        let meals: Vec<Meal> = serde_json::from_str(&list_json)?;
        Ok(meals)
    }
}
