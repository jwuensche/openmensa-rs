use getset::{ CopyGetters, Getters};
use serde::Deserialize;

/// Representation of possible prices of every category.
///
/// Not all prices have to be given.
#[derive(Deserialize, Getters, CopyGetters, Debug, Clone)]
pub struct Price {
    #[get_copy = "pub"]
    students: Option<f32>,
    #[get_copy = "pub"]
    employees: Option<f32>,
    #[get_copy = "pub"]
    others: Option<f32>,
    #[get_copy = "pub"]
    pupils: Option<f32>,
}
