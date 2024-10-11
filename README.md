# Wi-Fi-based Slack Status Updater

This Rust application automatically updates your Slack status based on your current Wi-Fi network. It's designed to set your status to "Working on-site" when connected to your office Wi-Fi and "Working remotely" when on any other network.

## Features

- Automatically detects your current Wi-Fi network
- Updates Slack status based on network connection
- Customizable status messages and emojis
- Can run as a systemd service for continuous background operation
- Can be run manually for one-time updates

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager
- A Slack workspace with permissions to update your status
- Linux-based system with `nmcli` installed (for Wi-Fi detection)
- Systemd (for service management, optional)

## Slack Token Setup

This application requires a Slack token with the `users.profile:write` scope. To obtain this token:

1. Go to the [Slack API Apps page](https://api.slack.com/apps)
2. Create a new app or select an existing one
3. Navigate to "OAuth & Permissions" in the sidebar
4. Under "Scopes", add the `users.profile:write` scope to your Bot Token Scopes
5. Install or reinstall your app to your workspace
6. Copy the Bot User OAuth Token to use as your `WSS_SLACK_TOKEN`

Ensure you keep this token secure and do not share it publicly.

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/your-username/wifi-slack-status-updater.git
   cd wifi-slack-status-updater
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

### Running Manually

You can run the application manually for a one-time status update:

1. Set the required environment variables:

   ```
   export WSS_SLACK_TOKEN="your_slack_token_here"
   export WSS_OFFICE_WIFI="your_office_wifi_name_here"
   export WSS_ON_SITE_TEXT="Working on-site"
   export WSS_REMOTE_TEXT="Working remotely"
   export WSS_ON_SITE_EMOJI="office"
   export WSS_REMOTE_EMOJI="house"
   ```

   Replace the values with your actual Slack token (with the `users.profile:write` scope), office Wi-Fi name, and preferred status texts and emojis.

2. Run the application:

   ```
   ./target/release/wifi-slack-status-updater
   ```

   The application will check your current Wi-Fi, update your Slack status, and then exit.

### Installing as a Systemd Service

To have the application run continuously in the background and start automatically on system boot, you can install it as a systemd service:

1. Run the installation script with sudo:

   ```
   sudo ./scripts/install.sh
   ```

   The script will prompt you for:

   - The path to your compiled Rust binary
   - Your Slack token (ensure it has the `users.profile:write` scope)
   - Your office Wi-Fi name
   - The username to run the service under

2. The script will create a systemd service, enable it, and start it for you.

## Managing the Systemd Service

[The rest of the content remains the same as in the previous version]
