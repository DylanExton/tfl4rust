use serde::Serialize;

#[derive(Serialize)]
pub struct AirQuality {
    pub forecast_url: String,
    pub current_forecast: AirQualityForecast,
    pub future_forecast: AirQualityForecast,
}

#[derive(Serialize)]
pub struct AirQualityForecast {
    pub forecast_id: String,
    pub creation_date: String,
    pub valid_from: String,
    pub valid_to: String,
    pub forecast_band: String,
    pub summary: String,
    pub no2_band: String,
    pub o3_band: String,
    pub pm10_band: String,
    pub pm25_band: String,
    pub so2_band: String,
    pub forecast_text: String,
}

impl AirQualityForecast {
    pub fn empty() -> Self {
        Self {
            forecast_id: String::new(),
            creation_date: String::new(),
            valid_from: String::new(),
            valid_to: String::new(),
            forecast_band: String::new(),
            summary: String::new(),
            no2_band: String::new(),
            o3_band: String::new(),
            pm10_band: String::new(),
            pm25_band: String::new(),
            so2_band: String::new(),
            forecast_text: String::new(),
        }
    }
}