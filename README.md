# TradingView Webhook for Discord
This App creates a Service Instance in Google Cloud Run to translate alerts from TrandingView and post them in a channel in Discord via WeebHook integration.

## Before start you would need
* Fork the repo
* Have [TradingView](https://www.tradingview.com/) 
* Have your server in Discord with admin rights
* Create your Weebhook integration in Discord server and copy the URL an have it in an action secret named DISCORD_WEBHOOKS (see below how to use it)
* Have your actions secrets for the credentials to Google Cloud Run: GCP_RUN_CREDENTIALS (json file) and GOOGLE_PROJECT_ID

## Working with multiple discord webhooks in your secret DISCORD_WEBHOOKS

github actions can't manage json lists properly (I found out that in the hard way) in secrets.
This is why you will have to code the Json to be used as a secret.
Let's assume the following list
```json
[
    {
        "path" : "bitcoin",
        "url" : "http://your_webwook_url_bitcoin_alerts"
    },
    {
        "path" : "kaspa",
        "url" : "http://your_webwook_url_kaspa_alerts"
    }    
]
```

Now you need to code it:

```bash
export WEBHOOKS='[
    {
        "path" : "bitcoin",
        "url" : "http://your_webwook_url_bitcoin_alerts"
    },
    {
        "path" : "kaspa",
        "url" : "http://your_webwook_url_kaspa_alerts"
    }    
]
'
```

You can check if the env is working"
```bash
echo $WEBHOOKS
[ { "path" : "bitcoin", "url" : "http://your_webwook_url_bitcoin_alerts" }, { "path" : "kaspa", "url" : "http://your_webwook_url_kaspa_alerts" } ]
```

And then code it
```bash
printf '%s' "${WEBHOOKS}" | jq -sRr @uri
%5B%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20%22path%22%20%3A%20%22bitcoin%22%2C%0A%20%20%20%20%20%20%20%20%22url%22%20%3A%20%22http%3A%2F%2Fyour_webwook_url_bitcoin_alerts%22%0A%20%20%20%20%7D%2C%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20%22path%22%20%3A%20%22kaspa%22%2C%0A%20%20%20%20%20%20%20%20%22url%22%20%3A%20%22http%3A%2F%2Fyour_webwook_url_kaspa_alerts%22%0A%20%20%20%20%7D%20%20%20%20%0A%5D%0A
```

And copy and paste this whole coded line to the secret: `DISCORD_WEBHOOKS`

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
* Execute it:
    ```bash
    DISCORD_WEBHOOKS="%5B%0A%20%0A%5D%0A.." \ # ..rest of the coded content
    PORT="8080" RUST_LOG="info" ./target/debug/whistle
    ```
* Test it: `curl -X POST http://localhost:8080/webhook/bitcoin -H "Content-Type: application/json" -d '{"event": "Crossing trend line"}'`

## Test it locally with Dockers
* Clone the repo
* Build it: `docker build -t whistle:latest .`
* Execute it:
    ```bash
    d ocker run -d \
    -e DISCORD_WEBHOOKS="%5B%0A%20%0A%5D%0A.." \ # ..rest of the coded content
    -e RUST_LOG="debug" \
    -e PORT="8080" -p 8080:8080 whistle:latest
    ```
* Test it: `curl -X POST http://localhost:8080/webhook/bitcoin -H "Content-Type: application/json" -d '{"event": "Crossing trend line"}'`

