use getset::CopyGetters;
use serde::Deserialize;

/// Reprensentation of a day.
#[derive(Deserialize, CopyGetters, Debug, Clone)]
pub struct Day {
    /// Date strings are in the `YYYY-MM-DD` format.
    date: String,
    #[get_copy = "pub"]
    closed: bool,
}

impl Day {
    pub fn date(&self) -> String {
        self.date.clone()
    }
}
