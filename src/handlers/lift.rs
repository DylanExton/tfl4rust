// Handler for the lift disruptions

use axum::{extract::State, Json};
use crate::models::lifts::LiftDisruption;
use crate::requests::client::TflClients;
use crate::requests::lifts::get_tfl_lift_disruptions;

#[utoipa::path(
    get,
    path = "/lifts",
    description = "List of all Lift Disruptions at all TFL stations",
    responses(
        (status = 200, description = "Returns a JSON list of Lift Disruptions"),
    )
)]
pub async fn get_lift_disruptions(
    State(clients): State<TflClients>,
) -> Json<Vec<LiftDisruption>> {
    let resp: Vec<LiftDisruption> = get_tfl_lift_disruptions(&clients.unified).await;
    Json(resp)
}
