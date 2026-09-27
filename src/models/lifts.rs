use serde::Serialize;

#[derive(Serialize)]
pub struct LiftDisruption { 
    pub station_code: String,
    pub station: String,
    pub message: String,
}