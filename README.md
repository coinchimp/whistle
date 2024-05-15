# TradingView Webhook for Discord
This App creates a Service Instance in Google Cloud Run to translate alerts from TrandingView and post them in a channel in Discord via WeebHook integration.

## Before start you would need
* Fork the repo
* Have [TradingView](https://www.tradingview.com/) 
* Have your server in Discord with admin rights
* Create your Weebhook integration in Discord server and copy the URL an have it in an action secret named WEBHOOK_URL
* Have your actions secrets for the credentials to Google Cloud Run: GCP_RUN_CREDENTIALS (json file) and GOOGLE_PROJECT_ID

## How to install with google actions
* Fork this repo
* Add the actions secrets to your repo
* Do your `git clone` and then your `git push`, the workflow should take care of the rest. Check actions for log details in case it doesn't.

## After it's running in Google Cloud run
* Copy your service instance URL and add `/webhook` to it
* When you set the alert in TradingView create a JSON format like this:
```json
{ 
 "exchange" : "{{exchange}}", 
 "ticker" :  "{{ticker}}",
 "close" : "{{close}}",
 "open" : "{{open}}",
 "volume" : "{{volume}}",
 "event" : "Crossing ATH",
 "interval" : "{{interval}}"
}
```
* Then add the webhook URL to your notifications.

## Test it locally
Just do the standard stuff with Rust Apps
* Clone the repo
* Do a `cargo init`
* Then check dependencies to `Cargo.toml`
* use `cargo build --release` and use `cargo clean` before to remove any previous build results.
* Execute it: `WEBHOOK_URL="copy_webhook_URL_from_Discord_server" PORT="8080" RUST_LOG="info" ./target/debug/whistle`
* Test it: `curl -X POST http://localhost:8080/webhook -H "Content-Type: application/json" -d '{"event": "Crossing trend line"}'`

## Test it locally with Dockers
* Clone the repo
* Build it: `docker build -t whistle:latest .`
* Execute it: `docker run -d -e WEBHOOK_URL="copy_webhook_URL_from_Discord_server" -e RUST_LOG="debug" -e PORT="8080" -p 8080:8080 whistle:latest`
* Test it: `curl -X POST http://localhost:8080/webhook -H "Content-Type: application/json" -d '{"event": "Crossing trend line"}'`

