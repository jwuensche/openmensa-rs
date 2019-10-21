use chrono::{Date, Utc};
use http::method::Method;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use surf::Request;

use crate::error::RequestError;
use crate::BASE_URL;

#[derive(Deserialize, Debug)]
pub struct Day {
    date: String,
    closed: bool,
}

#[derive(Clone)]
pub struct DayRequest {
    start: Option<Date<Utc>>,
    canteen_id: u8,
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
    pub fn new(canteen_id: u8) -> Self {
        Self {
            start: None,
            canteen_id,
        }
    }

    pub fn with_start_date(mut self, start_date: Date<Utc>) -> Self {
        self.start = Some(start_date);
        self
    }

    pub async fn build(self) -> Result<Vec<Day>, RequestError> {
        let list_json = Request::new(
            Method::GET,
            url::Url::parse(format!("{}/canteens/{}/days", BASE_URL, self.canteen_id).as_str())?,
        )
        .body_json(&self)?
        .recv_string()
        .await?;
        let canteens: Vec<Day> = serde_json::from_str(&list_json)?;
        Ok(canteens)
    }
}
