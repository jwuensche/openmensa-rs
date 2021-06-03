use getset::{CopyGetters, Setters};
use serde::Deserialize;

/// Representation for geographic location given to each canteen.
#[derive(Deserialize, CopyGetters, Setters, Debug, Clone, Copy)]
pub struct CoordinatePair {
    #[get_copy = "pub"]
    #[set = "pub"]
    latitude: f64,
    #[get_copy = "pub"]
    #[set = "pub"]
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
#[derive(Deserialize, CopyGetters, Debug, Clone)]
pub struct Canteen {
    #[get_copy = "pub"]
    id: u16,
    name: String,
    city: String,
    address: String,
    #[get_copy = "pub"]
    coordinates: Option<CoordinatePair>,
}

impl Canteen {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn city(&self) -> String {
        self.city.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}
