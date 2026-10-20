#!/usr/bin/env bash
set -Eeuo pipefail

# Configuration
APP_NAME="rustu-cam"
CARGO_BIN_NAME="rustyCam"
INSTALL_DIR="$HOME/bin"
CONFIG_ROOT="${XDG_CONFIG_HOME:-$HOME/.config}"
CONFIG_DIR="$CONFIG_ROOT/rusty-cam"
CONFIG_FILE="$CONFIG_DIR/config.toml"
SYSTEMD_USER_DIR="$CONFIG_ROOT/systemd/user"
SERVICE_FILE="$SYSTEMD_USER_DIR/$APP_NAME.service"

PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

echo "==> 1. Building release binary..."
cargo build --release --bin "$CARGO_BIN_NAME"

echo "==> 2. Installing binary to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
install -m 755 "target/release/$CARGO_BIN_NAME" "$INSTALL_DIR/$APP_NAME"

echo "==> 3. Checking configuration..."
mkdir -p "$CONFIG_DIR"

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Generating default configuration..."

    "$INSTALL_DIR/$APP_NAME" &
    BIN_PID=$!
    sleep 1
    kill $BIN_PID 2>/dev/null || true

    echo -e "\nConfiguration created at:\n  $CONFIG_FILE\n"
    echo "Please edit the camera credentials, MAC address, interface, and subnet."
    echo "Then run this script again to install the service."
    exit 0
fi

echo "Configuration exists: $CONFIG_FILE"

echo "==> 4. Installing systemd user service..."
mkdir -p "$SYSTEMD_USER_DIR"

cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Camera Monitor Daemon
After=graphical-session.target
PartOf=graphical-session.target

[Service]
Type=simple
Environment="XDG_CONFIG_HOME=$CONFIG_ROOT"
ExecStart=%h/bin/$APP_NAME
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=graphical-session.target
EOF

echo "==> 5. Enabling service..."
systemctl --user daemon-reload
systemctl --user enable "$APP_NAME.service"

if systemctl --user is-active --quiet graphical-session.target; then
    echo "Graphical session is active. Starting service..."
    systemctl --user restart "$APP_NAME.service"
else
    echo "Service enabled. It will start when graphical-session.target activates."
fi

echo "=========================================================="
echo "✅ Installation complete!"
echo "Status:  systemctl --user status $APP_NAME"
echo "Logs:    journalctl --user -u $APP_NAME -f"
echo "=========================================================="