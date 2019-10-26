use getset::CopyGetters;
use serde::Deserialize;

use crate::price::Price;

/// Representation of a single meal.
#[derive(Deserialize, CopyGetters, Debug, Clone)]
pub struct Meal {
    #[get_copy = "pub"]
    id: u32,
    name: String,
    notes: Vec<String>,
    prices: Price,
}

// TODO
// At the moment this is all cloned for simplicity and to work better with the data.
// This may be changed later on, to reduce the amount of cloning operations.

impl Meal {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn notes(&self) -> Vec<String> {
        self.notes.clone()
    }

    pub fn prices(&self) -> Price {
        self.prices.clone()
    }
}
