use serde_json::{json, Value};
use warp::{Filter, http::StatusCode, Rejection, Reply, reply};
use std::env;
use reqwest;
use log::{info, error};
use urlencoding::decode;
use std::borrow::Cow;

async fn send_to_discord(path: String, data: Value) -> Result<impl Reply, Rejection> {
    let discord_webhooks_env = env::var("DISCORD_WEBHOOKS").unwrap_or_else(|_| String::from("[]"));
    info!("DISCORD_WEBHOOKS (encoded): {}", discord_webhooks_env); //   Debugging line to log the content of DISCORD_WEBHOOKS

    let decoded_webhooks: Cow<str> = decode(&discord_webhooks_env).unwrap_or_else(|_| String::from("[]").into());
    info!("DISCORD_WEBHOOKS (decoded): {}", decoded_webhooks); //  Debugging line to log the decoded content of DISCORD_WEBHOOKS

    // Ensure that the decoded JSON is properly formatted
    let webhooks: Value = match serde_json::from_str(&decoded_webhooks) {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to parse DISCORD_WEBHOOKS: {}", err);
            error!("Decoded DISCORD_WEBHOOKS value: {}", decoded_webhooks);
            json!([])
        }
    };
    
    info!("Parsed webhooks: {:?}", webhooks); // Debugging line to log parsed webhooks

    let webhook_url = webhooks
        .as_array()
        .and_then(|arr| arr.iter().find(|obj| obj["path"] == path))
        .and_then(|obj| obj["url"].as_str());

    match webhook_url {
        Some(url) => {
            info!("Using webhook URL: {}", url);
            let client = reqwest::Client::new();

            info!("Received data: {}", data);

            let payload = if let Some(map) = data.as_object() {
                let exchange = map.get("exchange").and_then(Value::as_str).unwrap_or("");
                let ticker = map.get("ticker").and_then(Value::as_str).unwrap_or("");
                let close = map.get("close").and_then(Value::as_str).unwrap_or("");
                let open = map.get("open").and_then(Value::as_str).unwrap_or("");
                let volume = map.get("volume").and_then(Value::as_str).unwrap_or("");
                let event = map.get("event").and_then(Value::as_str).unwrap_or("");
                let interval = map.get("interval").and_then(Value::as_str).unwrap_or("");

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

            match client.post(url).json(&payload).send().await {
                Ok(_) => {
                    info!("Message successfully sent to Discord.");
                    Ok(reply::with_status("Content sent to Discord", StatusCode::OK))
                },
                Err(e) => {
                    error!("Failed to send message to Discord: {:?}", e);
                    Err(warp::reject::reject())
                }
            }
        },
        None => {
            error!("No valid webhook URL found for path: {}", path);
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
