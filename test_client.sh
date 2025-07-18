#!/bin/bash

# MLS Enhanced Client Test Script
# This script demonstrates the client functionality and connection status

echo "=== MLS Enhanced Client Test ==="
echo

# Check if the client builds successfully
echo "1. Building the client..."
if cargo build --release; then
    echo "✓ Build successful"
else
    echo "✗ Build failed"
    exit 1
fi

echo

# Test configuration
echo "2. Testing configuration..."
if [ -f "config.json" ]; then
    echo "✓ Configuration file exists"
    cat config.json
else
    echo "✓ Configuration will be created on first run"
fi

echo

# Test connection status
echo "3. Testing connection status..."
echo "The client will show connection status when started:"
echo "- 'Connected to MLS service at <address>' if service is available"
echo "- 'Disconnected from MLS service at <address>' if service is unavailable"
echo

# Test commands
echo "4. Available commands:"
echo "   create <group_name> - Create a new group"
echo "   join <group_id>     - Join an existing group"
echo "   send <message>      - Send a message to active group"
echo "   status             - Check MLS service connection"
echo "   settings           - Open settings"
echo "   help               - Show help"
echo "   quit               - Exit application"
echo

# Test navigation
echo "5. Navigation keys:"
echo "   ↑/↓               - Navigate between groups"
echo "   PageUp/PageDown   - Scroll messages"
echo "   c                 - Enter command mode"
echo "   m                 - Enter message mode"
echo "   s                 - Open settings"
echo "   h                 - Show help"
echo "   q                 - Quit"
echo

# Troubleshooting tips
echo "6. Troubleshooting:"
echo "   - If groups show '(1)' member count:"
echo "     * Check MLS service connection with 'status' command"
echo "     * Ensure delivery service is running"
echo "     * Groups work locally when disconnected"
echo
echo "   - If cannot join groups:"
echo "     * Verify MLS service is running"
echo "     * Check service address in settings"
echo "     * Use 'status' command to test connection"
echo

echo "=== Test Complete ==="
echo
echo "To run the client:"
echo "  cargo run --release"
echo
echo "To test with MLS service:"
echo "  1. Start your MLS delivery service"
echo "  2. Update settings with service address"
echo "  3. Use 'status' command to verify connection"
echo "  4. Create/join groups for shared messaging" 