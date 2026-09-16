mod handlers;
mod requests;

use axum::{routing::get};
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

#[tokio::main]
async fn main() {
    let (app, openapi) = OpenApiRouter::new()
        .routes(routes!(handlers::base::hello))
        .routes(routes!(handlers::base::app_info))
        .split_for_parts();

    // Route for the auto-discovering endpoints route to print the available enpoints that are registered above
    let app = app
        .route("/endpoints", get(handlers::base::endpoints))
        .with_state(openapi);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}