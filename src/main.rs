mod handlers;
mod requests;
mod models;

use axum::{extract::FromRef, routing::get};
use requests::client::TflClients;
use utoipa::openapi::OpenApi;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

#[derive(Clone)]
struct AppState {
    clients: TflClients,
    openapi: OpenApi,
}

impl FromRef<AppState> for TflClients {
    fn from_ref(state: &AppState) -> Self {
        state.clients.clone()
    }
}

impl FromRef<AppState> for OpenApi {
    fn from_ref(state: &AppState) -> Self {
        state.openapi.clone()
    }
}

#[tokio::main]
async fn main() {
    let clients = TflClients::from_env();

    let (app, openapi) = OpenApiRouter::new()
        .routes(routes!(handlers::base::hello))
        .routes(routes!(handlers::base::app_info))
        .routes(routes!(handlers::lift::get_lift_disruptions))
        .routes(routes!(handlers::air_quality::get_air_quality))
        .split_for_parts();

    let state = AppState { clients, openapi };

    // Route for the auto-discovering endpoints route to print the available enpoints that are registered above
    let app = app
        .route("/endpoints", get(handlers::base::endpoints))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
