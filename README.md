# Wi-Fi-based Slack Status Updater

This Rust application automatically updates your Slack status based on your current Wi-Fi network. It's designed to set your status to "Working on-site" when connected to your office Wi-Fi and "Working remotely" when on any other network.

## Features

- Automatically detects your current Wi-Fi network
- Updates Slack status based on network connection
- Customizable status messages and emojis
- Easy to configure with environment variables

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager
- A Slack workspace with permissions to update your status
- Linux-based system with `nmcli` installed (for Wi-Fi detection)

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

## Configuration

Set the following environment variables:

- `WSS_SLACK_TOKEN`: Your Slack OAuth token (should start with "xoxp-")
- `WSS_OFFICE_WIFI`: The exact name of your office Wi-Fi network
- `WSS_ON_SITE_TEXT` (optional): Custom text for on-site status (default: "Working on-site")
- `WSS_REMOTE_TEXT` (optional): Custom text for remote status (default: "Working remotely")
- `WSS_ON_SITE_EMOJI` (optional): Custom emoji for on-site status (default: "office")
- `WSS_REMOTE_EMOJI` (optional): Custom emoji for remote status (default: "house")

You can set these variables in your shell or create a `.env` file in the project root.

## Usage

Run the application:

```
cargo run --release
```

The application will detect your current Wi-Fi network and update your Slack status accordingly.

## Running as a Service

To have this application run automatically on system startup, you can set it up as a systemd service on Linux systems. Create a file named `wifi-slack-status.service` in `/etc/systemd/system/` with the following content:

```
[Unit]
Description=Wi-Fi Slack Status Updater
After=network.target

[Service]
ExecStart=/path/to/your/compiled/binary
Restart=always
User=your_username
Environment="WSS_SLACK_TOKEN=your_slack_token"
Environment="WSS_OFFICE_WIFI=your_office_wifi_name"
# Add other environment variables as needed

[Install]
WantedBy=multi-user.target
```

Replace the placeholders with your actual values. Then enable and start the service:

```
sudo systemctl enable wifi-slack-status.service
sudo systemctl start wifi-slack-status.service
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for providing excellent libraries and tools.
- Slack for their API documentation and support.
