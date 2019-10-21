use chrono::Date;
use surf::get;
#[macro_use]
extern crate failure;

mod canteen;
mod day;
mod error;
mod meal;
mod price;

pub use canteen::{Canteen, CanteenRequest};
pub use day::{Day, DayRequest};
pub use error::RequestError;
pub use meal::{DailyMeals, Meal, MealRequest};

const BASE_URL: &str = "https://openmensa.org/api/v2";

pub async fn req_canteens() -> Result<Vec<Canteen>, RequestError> {
    let list_json = get(format!("{}/canteens", BASE_URL)).recv_string().await?;
    let canteens: Vec<Canteen> = serde_json::from_str(&list_json).expect("Could not derive body");
    Ok(canteens)
}

pub async fn req_opening_days(canteen: Canteen) -> Result<Vec<Day>, RequestError> {
    let day_list = get(format!("{}/canteens/{}/days", BASE_URL, canteen.id))
        .recv_string()
        .await?;
    println!("{:?}", day_list);
    let days: Vec<Day> = serde_json::from_str(&day_list).expect("Could not deseriaize");
    Ok(days)
}

pub async fn req_meals(
    canteen: Canteen,
    _date: Date<chrono::Utc>,
) -> Result<Vec<DailyMeals>, RequestError> {
    let meal_list = get(format!("{}/canteens/{}/meals", BASE_URL, canteen.id))
        .recv_string()
        .await?;
    let meals: Vec<DailyMeals> = serde_json::from_str(&meal_list).expect("Could not deseriaize");
    Ok(meals)
}
