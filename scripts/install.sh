#!/bin/bash

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Prompt for necessary information
read -p "Enter the path to your Rust binary: " binary_path
read -p "Enter your Slack token: " slack_token
read -p "Enter your office Wi-Fi name: " office_wifi
read -p "Enter the username to run the service: " username

# Create the service file
cat <<EOF >/etc/systemd/system/wifi-slack-status@.service
[Unit]
Description=Wi-Fi Slack Status Updater
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=$binary_path
Restart=always
RestartSec=10
User=%i
Environment="WSS_SLACK_TOKEN=$slack_token"
Environment="WSS_OFFICE_WIFI=$office_wifi"
Environment="WSS_ON_SITE_TEXT=Working on-site"
Environment="WSS_REMOTE_TEXT=Working remotely"
Environment="WSS_ON_SITE_EMOJI=office"
Environment="WSS_REMOTE_EMOJI=house"

[Install]
WantedBy=multi-user.target
EOF

# Set correct permissions
chmod 644 /etc/systemd/system/wifi-slack-status@.service

# Reload systemd to recognize the new service
systemctl daemon-reload

# Enable and start the service
systemctl enable wifi-slack-status@$username.service
systemctl start wifi-slack-status@$username.service

echo "Service installed, enabled, and started for user $username"
