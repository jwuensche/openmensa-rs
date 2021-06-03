use getset::{CopyGetters, Getters};
use serde::Deserialize;

/// Reprensentation of a day.
#[derive(Deserialize, CopyGetters, Getters, Debug, Clone)]
pub struct Day {
    /// Date strings are in the `YYYY-MM-DD` format.
    #[get = "pub"]
    date: String,
    #[get_copy = "pub"]
    closed: bool,
}
