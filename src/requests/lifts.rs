// Request to the Lifts v2 TFL API
// API Docs: https://api-portal.tfl.gov.uk/api-details#api=Disruptions-Lifts-v2
// Serializes the response to TflLiftDisruption and returns LiftDisruption
use crate::models::lifts::LiftDisruption;
use crate::requests::client::TflClient;
use crate::requests::models;
use models::lifts::TflLiftDisruption;
use utoipa::r#gen::serde_json;

pub async fn get_tfl_lift_disruptions(client: &TflClient) -> Vec<LiftDisruption> {
    let resp = client.get_text("/Disruptions/Lifts/v2/").await;
    let json_resp: Vec<TflLiftDisruption> =
        serde_json::from_str::<Vec<TflLiftDisruption>>(&resp).expect("REASON");
    println!(
        "Retrieved {:#?} lift disruptions from TFL....",
        json_resp.len()
    );
    let mut ret_disruptions: Vec<LiftDisruption> = Vec::new();
    for disruption in json_resp {
        ret_disruptions.push(transform_disruption(
            disruption.station_id,
            disruption.message,
        ));
    }
    ret_disruptions
}

fn transform_disruption(station_code: String, message: String) -> LiftDisruption {
    let (station, message) = message.split_once(':').unwrap_or(("", &message));
    let station = title_case(station.trim());

    // Strip the "Call us..." message at the end if it exists
    let message = message
        .split("Call us on")
        .next()
        .unwrap_or(message)
        .split("Call ")
        .next()
        .unwrap_or(message)
        .trim()
        .to_string();

    let disruption = LiftDisruption {
        station_code,
        station,
        message,
    };
    return disruption;
}

fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
