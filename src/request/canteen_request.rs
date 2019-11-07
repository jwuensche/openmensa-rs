use crate::error::RequestError;
use crate::BASE_URL;
use http::method::Method;
use serde::{ser::SerializeMap, Serialize, Serializer};
use surf::Request;

use crate::canteen::{Canteen, CoordinatePair};

/// Struct to create a request receiving a list of canteens fitting the specified parameters.
///
/// This request does not require any additional information and can be stated as is to retrieve a list of all canteens.
/// # Example
///
/// ```rust
/// use openmensa_rs::request::CanteenRequest;
/// use openmensa_rs::CoordinatePair;
///
/// #[tokio::main]
/// async fn main() {
///     let near_canteens = CanteenRequest::new()
///         .with_near_coordinates(
///             CoordinatePair::new(
///                 52.1396188273019,
///                 11.6475999355316,
///             )
///         )
///         .build()
///         .await
///         .unwrap();
/// }
/// ```
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
                Some(pos.latitude())
            } else {
                None
            }
        };
        let lng = {
            if let Some(pos) = &self.near {
                Some(pos.longitude())
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
    /// Returns a new instance of a request.
    /// If you want to perform the same request again, you do not need to set all parameters anew. Requests are clonable, so to not lose them after building just create a clone!
    /// ```rust
    /// # use openmensa_rs::request::CanteenRequest;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let my_super_complex_request = CanteenRequest::new();
    /// let result = my_super_complex_request.clone().build().await.unwrap();
    /// // And we can use it again!
    /// let result = my_super_complex_request.build().await.unwrap();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            near: None,
            distance: None,
            ids: None,
            has_coordinates: None,
        }
    }

    /// Specify a `CoordinatePair` to which the canteen should be near to.
    /// If not further specified only canteens in a range of up to 10 kilometers will be returned.
    /// If you want to specify the range too, have a look at `with_distance_to` method.
    pub fn with_near_coordinates(mut self, near: CoordinatePair) -> Self {
        self.near = Some(near);
        self
    }

    /// Specify a `CoordinatePair` and distance to which the canteen should be near too.
    pub fn with_distance_to(mut self, dst: f32, near: CoordinatePair) -> Self {
        self.distance = Some(dst);
        self.near = Some(near);
        self
    }

    /// Add an id that should be included in the response.
    ///
    /// Note: If atleast one id is specified, only these ids will be included in the response. Even if other canteens would also match all other parameters. The same is true vice versa.
    pub fn with_id<U: Into<u8>>(mut self, id: U) -> Self {
        if let None = self.ids {
            self.ids = Some(Vec::new());
        }
        if let Some(ref mut ids) = self.ids {
            ids.push(id.into());
        }
        self
    }

    /// Add multiple ids to the request.
    pub fn with_ids<T: std::iter::Iterator>(mut self, ids: T) -> Self
    where
        T::Item: Into<u8>,
    {
        for id in ids {
            self = self.with_id(id);
        }
        self
    }

    /// Return only canteens that have specified coordinates.
    pub fn with_coordinates_requisite(mut self) -> Self {
        self.has_coordinates = Some(true);
        self
    }

    /// Send the request and wait for the response.
    /// May return an error if the parameters cannot be serialized or the response cannot be deserialized.
    /// ```rust
    /// # use openmensa_rs::request::CanteenRequest;
    /// # use openmensa_rs::CoordinatePair;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let near_canteens = CanteenRequest::new()
    ///     .with_near_coordinates(
    ///         CoordinatePair::new(
    ///             52.1396188273019,
    ///             11.6475999355316,
    ///         )
    ///     )
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn build(self) -> Result<Vec<Canteen>, RequestError> {
        let list_json = Request::new(
            Method::GET,
            url::Url::parse(format!("{}/canteens", BASE_URL).as_str())?,
        )
        .set_query(&self)?
        .recv_string()
        .await?;
        let canteens: Vec<Canteen> = serde_json::from_str(&list_json)?;
        Ok(canteens)
    }
}