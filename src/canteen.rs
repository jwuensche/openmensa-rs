use crate::error::RequestError;
use crate::BASE_URL;
use http::method::Method;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use surf::Request;

#[derive(Deserialize, Debug, Clone)]
pub struct CoordinatePair {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Canteen {
    pub id: u8,
    pub name: String,
    pub city: String,
    pub address: String,
    pub coordinates: CoordinatePair,
}

#[derive(Clone)]
pub struct CanteenRequest {
    // near[lat], near[lng]
    near: Option<CoordinatePair>,
    // near[dist]
    distance: Option<f32>,
    // ids
    ids: Option<Vec<u8>>,
    // hasCoordinates
    has_coordinates: Option<bool>,
}

impl Serialize for CanteenRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lat = {
            if let Some(pos) = &self.near {
                Some(pos.latitude)
            } else {
                None
            }
        };
        let lng = {
            if let Some(pos) = &self.near {
                Some(pos.longitude)
            } else {
                None
            }
        };
        let ids = {
            if let Some(ref ids) = &self.ids {
                Some(
                    ids.iter()
                        .fold(String::default(), |acc, ent| format!("{},{}", acc, ent)),
                )
            } else {
                None
            }
        };
        let mut map = serializer.serialize_map(Some(5)).unwrap();
        map.serialize_entry("near[lat]", &lat)?;
        map.serialize_entry("near[lng]", &lng)?;
        map.serialize_entry("distance", &self.distance)?;
        map.serialize_entry("ids", &ids)?;
        map.serialize_entry("hasCoordinates", &self.has_coordinates)?;
        map.end()
    }
}

impl CanteenRequest {
    pub fn new() -> Self {
        Self {
            near: None,
            distance: None,
            ids: None,
            has_coordinates: None,
        }
    }

    pub fn with_near_coordinates(mut self, near: CoordinatePair) -> Self {
        self.near = Some(near);
        self
    }

    pub fn with_distance_to(mut self, dst: f32, near: CoordinatePair) -> Self {
        self.distance = Some(dst);
        self.near = Some(near);
        self
    }

    pub fn with_id(mut self, id: u8) -> Self {
        if let None = self.ids {
            self.ids = Some(Vec::new());
        }
        if let Some(ref mut ids) = self.ids {
            ids.push(id);
        }
        self
    }

    pub fn with_coordinates_requisite(mut self) -> Self {
        self.has_coordinates = Some(true);
        self
    }

    pub async fn build(self) -> Result<Vec<Canteen>, RequestError> {
        // TODO Match error
        let list_json = Request::new(
            Method::GET,
            url::Url::parse(format!("{}/canteens", BASE_URL).as_str())?,
        )
        .body_json(&self)?
        .recv_string()
        .await?;
        // let list_json = get(format!("{}/canteens", BASE_URL)).recv_string().await?;
        let canteens: Vec<Canteen> = serde_json::from_str(&list_json)?;
        Ok(canteens)
    }
}
