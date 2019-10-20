use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Price {
    students: Option<f32>,
    employees: Option<f32>,
    others: Option<f32>,
    pupils: Option<f32>,
}
