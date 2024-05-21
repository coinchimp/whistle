use serde_json::{json, Value};
use warp::{Filter, http::StatusCode, Rejection, Reply, reply}; // Include reply here for reply::with_status
use std::env;
use reqwest;
use log::{info, error}; // Correctly import logging functions directly from the log crate

// Handles the incoming webhook and forwards content to Discord.
async fn send_to_discord(path: String, data: Value) -> Result<impl Reply, Rejection> {
    let webhooks: Value = serde_json::from_str(&env::var("DISCORD_WEBHOOKS").unwrap_or_else(|_| String::from("[]")))
        .unwrap_or_else(|_| json!([]));

    let webhook_url = webhooks
        .as_array()
        .and_then(|arr| arr.iter().find(|obj| obj["path"] == path))
        .and_then(|obj| obj["url"].as_str())
        .unwrap_or("your_default_webhook_url_here");

    let client = reqwest::Client::new();

    info!("Received data: {}", data);

    let payload = if let Some(map) = data.as_object() {
        // Assuming the data is in the expected JSON format
        let exchange = map.get("exchange").and_then(Value::as_str).unwrap_or("");
        let ticker = map.get("ticker").and_then(Value::as_str).unwrap_or("");
        let close = map.get("close").and_then(Value::as_str).unwrap_or("");
        let open = map.get("open").and_then(Value::as_str).unwrap_or("");
        let volume = map.get("volume").and_then(Value::as_str).unwrap_or("");
        let event = map.get("event").and_then(Value::as_str).unwrap_or("");
        let interval = map.get("interval").and_then(Value::as_str).unwrap_or("");

        // Determine the color based on close and open prices
        let color = if close < open {
            16711680 // Red color in decimal (0xFF0000)
        } else {
            65280 // Green color in decimal (0x00FF00)
        };

        json!({
            "embeds": [{
                "author": {
                    "name": format!("Whistle: {} {} at {}", ticker, event, exchange),
                    "url": "https://github.com/coinchimp/whistle",
                    "icon_url": "https://raw.githubusercontent.com/coinchimp/whistle/main/assets/images/whistle.png"
                },
                "description": format!("Open: {}\nClose: {}\nInterval: {}\nVolume: {}\n", open, close, interval, volume),
                "color": color
            }]
        })
    } else {
        // If the data is not an object, treat it as plain text
        json!({
            "embeds": [{
                "author": {
                    "name": "Whistle: Text Notification",
                    "url": "https://github.com/coinchimp/whistle",
                    "icon_url": "https://raw.githubusercontent.com/coinchimp/whistle/main/assets/images/whistle.png"
                },
                "description": format!("Event: {}", data),
                "color": 16761035 // Pink color in decimal (0xFFC0CB)
            }]
        })        
    };

    match client.post(webhook_url).json(&payload).send().await {
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
        .and(warp::path::param())
        .and(warp::body::json::<Value>())
        .and_then(|path: String, data: Value| send_to_discord(path, data));

    let health_route = warp::get()
        .and(warp::path::end())
        .map(|| "Healthy");

    let routes = webhook_route.or(health_route)
        .recover(handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
