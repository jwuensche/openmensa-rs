use getset::{CopyGetters, Setters, Getters};
use serde::Deserialize;

/// Representation for geographic location given to each canteen.
#[derive(Deserialize, Getters, Setters, Debug, Clone, Copy)]
pub struct CoordinatePair {
    #[getset(get = "pub", set = "pub")]
    latitude: f64,
    #[getset(get = "pub", set = "pub")]
    longitude: f64,
}

impl CoordinatePair {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

/// Representation of a canteen.
#[derive(Deserialize, CopyGetters, Getters, Debug, Clone)]
pub struct Canteen {
    #[getset(get_copy = "pub")]
    id: u16,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    city: String,
    #[getset(get = "pub")]
    address: String,
    #[getset(get = "pub")]
    coordinates: Option<CoordinatePair>,
}
