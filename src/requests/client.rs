use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

const UNIFIED_BASE_URL: &str = "https://api.tfl.gov.uk";
const TRACKERNET_BASE_URL: &str = "https://api.tfl.gov.uk/TrackerNet";

const UNIFIED_API_KEY_ENV: &str = "TFL_UNIFIED_API_KEY";
const TRACKERNET_API_KEY_ENV: &str = "TFL_TRACKERNET_API_KEY";

/// A reusable HTTP client configured for a specific TFL API.
///
/// Each client has its own base URL and sends the matching API key in the
/// `app_key` header on every request.
#[derive(Clone)]
pub struct TflClient {
    client: Client,
    base_url: String,
}

impl TflClient {
    fn new(base_url: impl Into<String>, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        if !api_key.is_empty() {
            headers.insert(
                "app_key",
                HeaderValue::from_str(api_key).expect("API key must be a valid header value"),
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest Client");

        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// GET a path relative to this client's base URL and return the response body as text.
    pub async fn get_text(&self, path: &str) -> String {
        let url = self.url(path);
        self.client
            .get(url)
            .send()
            .await
            .expect("REASON")
            .text()
            .await
            .expect("REASON")
    }

    fn url(&self, path: &str) -> String {
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        format!("{}{path}", self.base_url)
    }
}

/// Holds the two TFL API clients shared across the application via Axum state.
///
/// Request handlers pick the client that matches the upstream API they need.
#[derive(Clone)]
pub struct TflClients {
    pub unified: TflClient,
    pub trackernet: TflClient,
}

impl TflClients {
    /// Build both clients from environment variables.
    ///
    /// - `TFL_UNIFIED_API_KEY` for the Unified API
    /// - `TFL_TRACKERNET_API_KEY` for the Trackernet API
    ///
    /// Missing keys are allowed (empty header); TFL still serves data at a lower rate limit.
    pub fn from_env() -> Self {
        let unified_key = env::var(UNIFIED_API_KEY_ENV).unwrap_or_default();
        let trackernet_key = env::var(TRACKERNET_API_KEY_ENV).unwrap_or_default();

        Self {
            unified: TflClient::new(UNIFIED_BASE_URL, &unified_key),
            trackernet: TflClient::new(TRACKERNET_BASE_URL, &trackernet_key),
        }
    }
}
