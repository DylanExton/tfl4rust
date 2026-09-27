use serde::Deserialize;

// Lift Disruption v2 Response
#[derive(Deserialize)]
pub struct TflLiftDisruption {
    #[serde(rename = "stationUniqueId")]
    pub station_id: String,
    pub message: String,
}