use serde_json::{json, Value};
use warp::{Filter, http::StatusCode, Rejection, Reply, reply}; // Include reply here for reply::with_status
use std::env;
use reqwest;
use log::{info, error}; // Correctly import logging functions directly from the log crate

// Handles the incoming webhook and forwards content to Discord.
async fn send_to_discord(data: Value) -> Result<impl Reply, Rejection> {
    let webhook_url = env::var("WEBHOOK_URL").unwrap_or_else(|_| String::from("your_webhook_url_here"));
    let client = reqwest::Client::new();
    let content = data.to_string();

    info!("Received data: {}", data);

    let payload = json!({
        "content": content
    });

    match client.post(&webhook_url).json(&payload).send().await {
        Ok(_) => {
            info!("Message successfully sent to Discord.");
            Ok(reply::with_status("Content sent to Discord", StatusCode::OK))
        },
        Err(e) => {
            error!("Failed to send message to Discord: {:?}", e);
            Err(warp::reject::reject())
        }
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    info!("Handling rejection: {:?}", err);
    Ok(reply::with_status("Not found", StatusCode::NOT_FOUND))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();

    let webhook_route = warp::post()
        .and(warp::path("webhook"))
        .and(warp::body::json::<Value>())
        .and_then(send_to_discord);

    let health_route = warp::get()
        .and(warp::path::end())
        .map(|| "Healthy");

    let routes = webhook_route.or(health_route)
        .recover(handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
