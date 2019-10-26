use getset::Getters;
use serde::Deserialize;

/// Representation of possible prices of every category.
///
/// Not all prices have to be given.
#[derive(Deserialize, Getters, Debug, Clone)]
pub struct Price {
    #[get = "pub"]
    students: Option<f32>,
    #[get = "pub"]
    employees: Option<f32>,
    #[get = "pub"]
    others: Option<f32>,
    #[get = "pub"]
    pupils: Option<f32>,
}
