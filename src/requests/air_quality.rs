// Request to the AirQuality TFL API
// API Docs: https://api-portal.tfl.gov.uk/api-details#api=AirQuality&operation=AirQuality_Get
// Serializes the response to TflAirQuality and returns AirQuality
use crate::requests::client::TflClient;
use crate::models::air_quality::{AirQuality, AirQualityForecast};
use utoipa::r#gen::serde_json;
use crate::requests::models::air_quality::TflAirQuality;

pub async fn get_tfl_air_quality(client: &TflClient) -> AirQuality {
    let resp = client.get_text("/AirQuality/").await;
    let json_resp: TflAirQuality =
        serde_json::from_str::<TflAirQuality>(&resp).expect("REASON");
    
    transform_air_quality(json_resp)
}

fn transform_air_quality(tfl_resp: TflAirQuality) -> AirQuality {
    let forecast_url = tfl_resp.forecast_url;
    let forecasts = tfl_resp.forecasts;
    let mut current_forecast: AirQualityForecast = AirQualityForecast::empty();
    let mut future_forecast: AirQualityForecast = AirQualityForecast::empty();

    for forecast in forecasts {
        if forecast.forecast_type == "Current"{
            current_forecast.forecast_id = forecast.forecast_id;
            current_forecast.forecast_band = forecast.forecast_band;
            current_forecast.creation_date = forecast.created_at;
            current_forecast.forecast_text = forecast.forecast_text;
            current_forecast.summary = forecast.forecast_summary;
            current_forecast.no2_band = forecast.no2_band;
            current_forecast.o3_band = forecast.o3_band;
            current_forecast.pm10_band = forecast.pm10_band;
            current_forecast.pm25_band = forecast.pm25_band;
            current_forecast.so2_band = forecast.so2_band;
            current_forecast.valid_from = forecast.valid_from;
            current_forecast.valid_to = forecast.valid_to;
        }
        else if forecast.forecast_type == "Future"{
            future_forecast.forecast_id = forecast.forecast_id;
            future_forecast.forecast_band = forecast.forecast_band;
            future_forecast.creation_date = forecast.created_at;
            future_forecast.forecast_text = forecast.forecast_text;
            future_forecast.summary = forecast.forecast_summary;
            future_forecast.no2_band = forecast.no2_band;
            future_forecast.o3_band = forecast.o3_band;
            future_forecast.pm10_band = forecast.pm10_band;
            future_forecast.pm25_band = forecast.pm25_band;
            future_forecast.so2_band = forecast.so2_band;
            future_forecast.valid_from = forecast.valid_from;
            future_forecast.valid_to = forecast.valid_to;
            }
    }
    AirQuality{
        forecast_url,
        current_forecast,
        future_forecast,
    }
}