use chrono::{Date, Utc};

use crate::error::RequestError;
use crate::Meal;
use crate::BASE_URL;

/// Struct to create requests returning meals on a specific day in one canteen.
///
/// A list of meals is here returned all are offered at that day. For more information about the `Meal` have a look at its struct.
/// # Example
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use openmensa_rs::request::MealRequest;
///
/// let canteen_id = 1;
/// let meals = MealRequest::new(canteen_id, chrono::Date::from_utc(chrono::NaiveDate::from_ymd(2019, 11, 11), chrono::Utc))
///     .build()
///     .await
///     .unwrap();
/// # })
/// ```
pub struct MealRequest {
    canteen_id: u16,
    date: Date<Utc>,
}

impl MealRequest {
    /// Return a new instance of `MealRequest`.
    pub fn new(canteen_id: u16, date: Date<Utc>) -> Self {
        Self { canteen_id, date }
    }
    // pub fn with_meal_id(mut self, meal_id: u16) -> Self {
    //     self.meal_id = Some(meal_id);
    //     self
    // }

    /// Send the request and wait for the response.
    /// May return an error if the parameters cannot be serialized or the response cannot be deserialized.
    ///
    /// # Example
    /// ```rust
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use openmensa_rs::request::MealRequest;
    ///
    /// let canteen_id = 1;
    /// let offered_meals = MealRequest::new(canteen_id, chrono::Date::from_utc(chrono::NaiveDate::from_ymd(2019, 11, 11), chrono::Utc))
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # })
    /// ```
    pub async fn build(self) -> Result<Vec<Meal>, RequestError> {
        let list_json = reqwest::Client::new()
            .get(url::Url::parse(
                format!(
                    "{}/canteens/{}/days/{}/meals",
                    BASE_URL,
                    self.canteen_id,
                    self.date.format("%Y-%m-%d").to_string()
                )
                .as_str(),
            )?)
            .send()
            .await?
            .text()
            .await?;
        let meals: Vec<Meal> = serde_json::from_str(&list_json)?;
        Ok(meals)
    }
}
