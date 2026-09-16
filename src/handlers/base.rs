/// This file contains the handlers for more utility function than serving a purpose for the TFL API
/// wrapper that this program is
/// These will be used to test parts of the app and return information on the app itself rather than
/// actual information from TFL

use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
use utoipa::{
    openapi::{OpenApi},
};
use utoipa::openapi::Responses;

#[derive(Serialize)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub description: Option<String>,
    pub responses: Responses,
}

#[derive(Serialize)]
pub struct AppInfo{
    pub app_name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub url: String,
}

/// Returns information about all registered API endpoints.
/// Uses the OpenAPI metadata that is added to each endpoint to populate a JSON response list of endpoints
/// This one itself is never registered in main.rs therefore will not show on the /endpoints list
#[utoipa::path(
    get,
    path = "/endpoints",
    description = "List of all registered endpoints with OpenAPI/Utoipa Metadata",
    responses(
        (status = 200, description = "Returns a JSON list of Endpoints and File Metadata")
    )
)]
pub async fn endpoints(State(openapi): State<OpenApi>, ) -> Json<Vec<Endpoint>> {
    let mut endpoints = Vec::new();

    for (path, item) in &openapi.paths.paths {
        let methods = [
            ("GET", &item.get),
            ("POST", &item.post),
            ("PUT", &item.put),
            ("DELETE", &item.delete),
            ("PATCH", &item.patch),
            ("HEAD", &item.head),
            ("OPTIONS", &item.options),
            ("TRACE", &item.trace),
        ];

        for (method, operation) in methods {
            if let Some(operation) = operation {
                endpoints.push(Endpoint {
                    method: method.to_string(),
                    path: path.clone(),
                    description: operation.description.clone(),
                    responses: operation.responses.clone(),
                });
            }
        }
    }
    Json(endpoints)
}

#[utoipa::path(
    get,
    path = "/hello",
    description = "Check if server is active",
    responses(
        (status = 200, description = "Returns a Server is Active message")
    )
)]
pub async fn hello() -> &'static str {
    "Server is Active!"
}

#[utoipa::path(
    get,
    path = "/app_info",
    description = "Get version information about the application",
    responses(
    (status = 200, description = "Returns the application version information")
    )
)]
pub async fn app_info() -> Json<AppInfo> {
    Json(AppInfo{
        app_name: "tfl4rust".to_string(),
        version: "0.0.1".to_string(),
        author: "Dylan Exton".to_string(),
        description: "A Rust powered HTTP Server that connects to the TFL API's various endpoints and transforms the responses into more useful JSON for other apps to ingest. Simplifying the API calls and making it easier to access the data from TFL".to_string(),
        url: "https://github.com/DylanExton/tfl4rust".to_string()
    })
}