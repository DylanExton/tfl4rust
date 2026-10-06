use serde::Deserialize;

// Air Quality API Response
#[derive(Deserialize)]
pub struct TflAirQualityForecast {
    #[serde(rename = "forecastType")]
    pub forecast_type: String,
    #[serde(rename = "forecastID")]
    pub forecast_id: String,
    #[serde(rename = "publishedDate")]
    pub created_at: String,
    #[serde(rename = "fromDate")]
    pub valid_from: String,
    #[serde(rename = "toDate")]
    pub valid_to: String,
    #[serde(rename = "forecastBand")]
    pub forecast_band: String,
    #[serde(rename = "forecastSummary")]
    pub forecast_summary: String,
    #[serde(rename = "nO2Band")]
    pub no2_band: String,
    #[serde(rename = "o3Band")]
    pub o3_band: String,
    #[serde(rename = "pM10Band")]
    pub pm10_band: String,
    #[serde(rename = "pM25Band")]
    pub pm25_band: String,
    #[serde(rename = "sO2Band")]
    pub so2_band: String,
    #[serde(rename = "forecastText")]
    pub forecast_text: String,

}

#[derive(Deserialize)]
pub struct TflAirQuality {
    #[serde(rename = "updatePeriod")]
    pub update_period: String,
    #[serde(rename = "updateFrequency")]
    pub update_freqency: String,
    #[serde(rename = "disclaimerText")]
    pub disclaimer: String,
    #[serde(rename = "forecastURL")]
    pub forecast_url: String,
    #[serde(rename = "currentForecast")]
    pub forecasts: Vec<TflAirQualityForecast>,
}