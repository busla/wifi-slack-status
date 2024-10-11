use std::process::Command;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;
use std::error::Error;
use std::env;

#[derive(Debug)]
struct SlackStatus {
    status_text: String,
    status_emoji: String,
}

fn get_current_wifi() -> Option<String> {
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .ok()?;
    if !output.status.success() {
        eprintln!("Failed to execute nmcli command");
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("yes:") {
            return line.split(':').nth(1).map(|s| s.to_string());
        }
    }
    eprintln!("No active Wi-Fi network found");
    None
}

fn update_slack_status(slack_token: &str, status: &SlackStatus) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://slack.com/api/users.profile.set";

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {}", slack_token))?,
    );

    let body = json!({
        "profile": {
            "status_text": status.status_text,
            "status_emoji": status.status_emoji,
            "status_expiration": 0
        }
    });

    println!("Sending request to Slack API with body: {}", serde_json::to_string_pretty(&body)?);

    let response = client.post(url)
        .headers(headers)
        .json(&body)
        .send()?;

    println!("Slack API response status: {}", response.status());

    let response_text = response.text()?;
    println!("Full Slack API response: {}", response_text);

    if response_text.contains("\"ok\":true") {
        println!("Slack status updated successfully.");
    } else {
        eprintln!("Failed to update Slack status: {}", response_text);
    }
    Ok(())
}

fn main() {
    println!("Starting Wi-Fi Slack Status updater");

    let slack_token = env::var("WSS_SLACK_TOKEN").expect("WSS_SLACK_TOKEN must be set");
    let target_wifi_network = env::var("WSS_OFFICE_WIFI").expect("WSS_OFFICE_WIFI must be set");
    let on_site_text = env::var("WSS_ON_SITE_TEXT").unwrap_or_else(|_| "Working on-site".to_string());
    let remote_text = env::var("WSS_REMOTE_TEXT").unwrap_or_else(|_| "Working remotely".to_string());
    let on_site_emoji = env::var("WSS_ON_SITE_EMOJI").unwrap_or_else(|_| "office".to_string());
    let remote_emoji = env::var("WSS_REMOTE_EMOJI").unwrap_or_else(|_| "house".to_string());

    println!("Environment variables loaded successfully");
    println!("Target Wi-Fi network: {}", target_wifi_network);

    let current_wifi = get_current_wifi();
    println!("Current Wi-Fi network: {:?}", current_wifi);

    let status = if current_wifi.as_deref() == Some(target_wifi_network.trim()) {
        println!("Matched target Wi-Fi network");
        SlackStatus {
            status_text: on_site_text,
            status_emoji: format!(":{}:", on_site_emoji.trim_matches(':')),
        }
    } else {
        println!("Did not match target Wi-Fi network");
        SlackStatus {
            status_text: remote_text,
            status_emoji: format!(":{}:", remote_emoji.trim_matches(':')),
        }
    };

    println!("Updating Slack status to: {:?}", status);

    if let Err(e) = update_slack_status(&slack_token, &status) {
        eprintln!("Error updating Slack status: {}", e);
    }
}
