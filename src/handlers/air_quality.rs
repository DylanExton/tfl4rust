// Handler for the air quality API

use axum::{extract::State, Json};
use crate::models::air_quality::AirQuality;
use crate::models::lifts::LiftDisruption;
use crate::requests::client::TflClients;
use crate::requests::air_quality::get_tfl_air_quality;

#[utoipa::path(
    get,
    path = "/air_quality",
    description = "Returns the current and forecasted air quality",
    responses(
        (status = 200, description = "Returns the current and forecasted air quality"),
    )
)]
pub async fn get_air_quality(
    State(clients): State<TflClients>,
) -> Json<AirQuality> {
    let resp: AirQuality = get_tfl_air_quality(&clients.unified).await;
    Json(resp)
}
