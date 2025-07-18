#!/bin/bash

# MLS Enhanced Client - Multi-Group Testing Script
# This script simulates multi-group messaging with three users across two groups

echo "=== MLS Enhanced Client Multi-Group Test ==="
echo "This script will test the multi-group messaging functionality"
echo "You'll need to run the MLS Delivery Service first"
echo ""

# Check if the MLS Delivery Service is running
echo "Checking if MLS Delivery Service is running on 127.0.0.1:8080..."
if ! nc -z 127.0.0.1 8080 2>/dev/null; then
    echo "❌ MLS Delivery Service is not running!"
    echo "Please start it first:"
    echo "  git clone https://github.com/krich11/mls-delivery-service"
    echo "  cd mls-delivery-service"
    echo "  cargo run"
    echo ""
    echo "Or use Docker:"
    echo "  docker run -p 8080:8080 mls-delivery-service"
    exit 1
fi

echo "✅ MLS Delivery Service is running"
echo ""

# Build the client
echo "Building MLS Enhanced Client..."
if ! cargo build --release --quiet; then
    echo "❌ Failed to build client"
    exit 1
fi
echo "✅ Client built successfully"
echo ""

# Create test configurations for three users
echo "Creating test configurations..."

# User 1 config
cat > config_user1.json << EOF
{
  "username": "alice",
  "delivery_service_address": "127.0.0.1:8080"
}
EOF

# User 2 config  
cat > config_user2.json << EOF
{
  "username": "bob",
  "delivery_service_address": "127.0.0.1:8080"
}
EOF

# User 3 config
cat > config_user3.json << EOF
{
  "username": "charlie",
  "delivery_service_address": "127.0.0.1:8080"
}
EOF

echo "✅ Test configurations created"
echo ""

# Create test scenarios
echo "=== Test Scenarios ==="
echo ""
echo "Scenario 1: Three users in one group"
echo "- Alice creates 'team-all'"
echo "- Bob and Charlie join 'team-all'"
echo "- All three exchange messages"
echo ""
echo "Scenario 2: Two users in second group"
echo "- Alice creates 'team-leads'"  
echo "- Bob joins 'team-leads'"
echo "- Alice and Bob exchange messages"
echo ""
echo "Scenario 3: Multi-group messaging"
echo "- Alice switches between groups"
echo "- Messages sent to correct active group"
echo "- Group isolation verified"
echo ""

echo "=== Manual Testing Instructions ==="
echo ""
echo "1. Start three terminal sessions:"
echo ""
echo "   Terminal 1 (Alice):"
echo "   cp config_user1.json config.json"
echo "   cargo run"
echo ""
echo "   Terminal 2 (Bob):"
echo "   cp config_user2.json config.json"
echo "   cargo run"
echo ""
echo "   Terminal 3 (Charlie):"
echo "   cp config_user3.json config.json"
echo "   cargo run"
echo ""

echo "2. Test Group Creation (Alice):"
echo "   create team-all"
echo "   create team-leads"
echo ""

echo "3. Test Group Joining (Bob and Charlie):"
echo "   # Note the group IDs from Alice's screen"
echo "   join <team-all-id>"
echo "   join <team-leads-id>  # Bob only"
echo ""

echo "4. Test Multi-Group Messaging:"
echo "   # Alice switches between groups using arrow keys"
echo "   # Send messages to different groups"
echo "   send Hello team-all!"
echo "   # Switch to team-leads"
echo "   send Hello team-leads!"
echo ""

echo "5. Verify Group Isolation:"
echo "   # Messages should only appear in correct groups"
echo "   # Charlie should only see team-all messages"
echo "   # Bob should see both group messages"
echo ""

echo "=== Automated Test Commands ==="
echo ""
echo "The following commands can be used for automated testing:"
echo ""

# Create a simple test script for each user
cat > test_alice.txt << EOF
create team-all
create team-leads
send Welcome to team-all!
send Welcome to team-leads!
EOF

cat > test_bob.txt << EOF
join team-all-id-here
join team-leads-id-here
send Hi from Bob in team-all!
send Hi from Bob in team-leads!
EOF

cat > test_charlie.txt << EOF
join team-all-id-here
send Hi from Charlie in team-all!
EOF

echo "Test command files created:"
echo "- test_alice.txt"
echo "- test_bob.txt"
echo "- test_charlie.txt"
echo ""

echo "=== Expected Results ==="
echo ""
echo "✅ Group Creation:"
echo "   - Alice successfully creates both groups"
echo "   - Group IDs are generated and displayed"
echo ""
echo "✅ Group Joining:"
echo "   - Bob joins both groups successfully"
echo "   - Charlie joins team-all successfully"
echo ""
echo "✅ Message Delivery:"
echo "   - Messages appear in correct groups only"
echo "   - Timestamps are displayed correctly"
echo "   - End-to-end encryption is transparent"
echo ""
echo "✅ Multi-Group Switching:"
echo "   - Alice can switch between groups"
echo "   - Active group is highlighted"
echo "   - Messages sent to correct active group"
echo ""
echo "✅ Group Isolation:"
echo "   - team-all messages: Alice, Bob, Charlie"
echo "   - team-leads messages: Alice, Bob only"
echo "   - No message leakage between groups"
echo ""

echo "=== Troubleshooting ==="
echo ""
echo "If you encounter issues:"
echo "1. Check USAGE.md for detailed troubleshooting"
echo "2. Enable debug logging: RUST_LOG=debug cargo run"
echo "3. Verify delivery service is running and accessible"
echo "4. Check that usernames are unique across instances"
echo "5. Ensure terminal supports Unicode for proper TUI display"
echo ""

echo "=== Cleanup ==="
echo "To clean up test files:"
echo "rm -f config_user*.json test_*.txt"
echo ""

echo "Ready to test! Follow the manual testing instructions above."