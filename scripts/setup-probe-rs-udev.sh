#!/usr/bin/env bash
set -euo pipefail

RULES_FILE="/etc/udev/rules.d/99-cmsis-dap.rules"
VENDOR_ID="2e8a"
PRODUCT_ID="000c"

if [[ $EUID -ne 0 ]]; then
  echo "This script must be run as root (use sudo)." >&2
  exit 1
fi

# Ensure plugdev group exists
if ! getent group plugdev >/dev/null 2>&1; then
  groupadd plugdev
fi

# Install udev rule for Raspberry Pi Debug Probe (CMSIS-DAP)
cat > "$RULES_FILE" <<RULE
SUBSYSTEM=="usb", ATTR{idVendor}=="$VENDOR_ID", ATTR{idProduct}=="$PRODUCT_ID", MODE="0666", GROUP="plugdev"
RULE

udevadm control --reload-rules
udevadm trigger

echo "Installed udev rules in $RULES_FILE"
echo "Add your user to plugdev group (if not already):"
echo "  sudo usermod -aG plugdev \"$SUDO_USER\""
echo "Then log out and back in for group change to take effect."
