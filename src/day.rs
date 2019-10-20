use serde::Deserialize;
use chrono::{Date, Local};
use surf::Request;
use http::method::Method;

use crate::error::RequestError;
use crate::BASE_URL;

#[derive(Deserialize, Debug)]
pub struct Day {
    date: String,
    closed: bool,
}

#[derive(Clone)]
pub struct DayRequest {
    start: Option<Date<Local>>,
    canteen_id: u8,
}

impl DayRequest {
    pub fn new(canteen_id: u8) -> Self {
        Self {
            start: None,
            canteen_id,
        }
    }

    pub fn with_start_date(mut self, start_date: Date<Local>) -> Self {
        self.start = Some(start_date);
        self
    }

    pub async fn build(self) -> Result<Vec<Day>, RequestError> {
        // TODO Match error 
        let list_json = Request::new(Method::GET, url::Url::parse(format!("{}/canteens/{}/days", BASE_URL, self.canteen_id).as_str()).unwrap())
            .body_json(&self)?.recv_string().await?;
        let canteens: Vec<Day> = serde_json::from_str(&list_json)?;
        Ok(canteens)
    }

}
