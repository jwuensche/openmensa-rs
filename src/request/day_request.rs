use chrono::{Date, Utc};
use serde::{ser::SerializeMap, Serialize, Serializer};

use crate::day::Day;
use crate::error::RequestError;
use crate::BASE_URL;

/// Struct to create and then issue requests to see a range of opening days.
/// # Example
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use openmensa_rs::request::DayRequest;
///
/// // As an example we just use the canteen with id `1` here
/// let req = DayRequest::new(1);
/// let list_of_days = req.build().await.unwrap();
/// println!("{:?}", list_of_days);
/// # })
/// ```
#[derive(Clone)]
pub struct DayRequest {
    start: Option<Date<Utc>>,
    canteen_id: u16,
}

impl Serialize for DayRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1)).unwrap();
        let date_string: Option<String> = {
            if let Some(date) = self.start {
                Some(date.format("%Y-%m-%d").to_string())
            } else {
                None
            }
        };
        map.serialize_entry("start", &date_string)?;
        map.end()
    }
}

impl DayRequest {
    /// Create a new instance to get opening days from a specific canteen.
    /// If you want to issue multiple requests, this struct implements `Clone` so before you issue a request simply `.clone()` if you need it later on.
    pub fn new(canteen_id: u16) -> Self {
        Self {
            start: None,
            canteen_id,
        }
    }

    /// Specfiy a start date from which opening days should be returned from.
    /// By default this is the current date, but you may specify another if you want e.g. to know the dates in a week or two.
    pub fn with_start_date(mut self, start_date: Date<Utc>) -> Self {
        self.start = Some(start_date);
        self
    }

    /// Send the request and wait for the response.
    /// May return an error if the parameters cannot be serialized or the response cannot be deserialized.
    ///
    /// # Example
    /// ```rust
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use openmensa_rs::request::DayRequest;
    ///
    /// let canteen_id = 1;
    /// let open_days = DayRequest::new(canteen_id)
    ///     .with_start_date(
    ///         chrono::Utc::today()
    ///     )
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # })
    /// ```
    pub async fn build(self) -> Result<Vec<Day>, RequestError> {
        let list_json = reqwest::Client::new()
            .get(url::Url::parse(
                format!("{}/canteens/{}/days", BASE_URL, self.canteen_id).as_str(),
            )?)
            .query(&self)
            .send()
            .await?
            .text()
            .await?;
        let canteens: Vec<Day> = serde_json::from_str(&list_json)?;
        Ok(canteens)
    }
}
