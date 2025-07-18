# MLS Delivery Service Setup Guide

This guide explains how to set up a real MLS delivery service to work with the MLS Enhanced Client for actual multi-client group messaging.

## Overview

The MLS Enhanced Client now supports real WebSocket communication with MLS delivery services. This enables:
- **Real group creation and joining** across multiple clients
- **Secure message delivery** using MLS protocol
- **Proper member management** with actual group membership
- **End-to-end encryption** for all communications

## MLS Delivery Service Requirements

### Protocol Specification

The client expects a WebSocket-based MLS delivery service that implements the following message protocol:

#### Client to Server Messages

```json
{
  "type": "create_group",
  "group_id": "string",
  "group_info": "base64_encoded_bytes"
}

{
  "type": "join_group", 
  "group_id": "string",
  "key_package": "base64_encoded_bytes"
}

{
  "type": "send_message",
  "group_id": "string", 
  "message": "base64_encoded_bytes"
}

{
  "type": "fetch_messages",
  "group_id": "string"
}

{
  "type": "publish_key_package",
  "key_package": "base64_encoded_bytes"
}

{
  "type": "fetch_key_packages",
  "identity": "string"
}
```

#### Server to Client Messages

```json
{
  "type": "group_created",
  "group_id": "string",
  "success": true,
  "error": null
}

{
  "type": "group_joined",
  "group_id": "string", 
  "welcome_message": "base64_encoded_welcome_bytes",
  "error": null
}

{
  "type": "message_received",
  "group_id": "string",
  "message": "base64_encoded_message"
}

{
  "type": "key_package_published",
  "success": true,
  "error": null
}

{
  "type": "key_packages_fetched",
  "identity": "string",
  "key_packages": ["base64_encoded_key_package"]
}

{
  "type": "error",
  "message": "error_description"
}
```

## Implementation Options

### Option 1: Use Existing MLS Delivery Service

If you have access to an existing MLS delivery service:

1. **Get the service address** (e.g., `ws://localhost:8080` or `wss://your-service.com`)
2. **Update client settings**:
   ```bash
   # In the client, use 's' key to open settings
   # Set Delivery Service Address to your service URL
   ```
3. **Test connection** with `status` command

### Option 2: Build Your Own MLS Delivery Service

#### Prerequisites
- Rust 1.70+
- WebSocket server implementation
- MLS protocol knowledge

#### Basic Implementation Structure

```rust
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("MLS Delivery Service listening on {}", addr);
    
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    handle_message(&mut ws_sender, value).await;
                }
            }
            _ => {}
        }
    }
}

async fn handle_message(
    sender: &mut futures_util::stream::SplitSink<_, _>,
    msg: Value
) {
    match msg["type"].as_str() {
        Some("create_group") => {
            // Handle group creation
            let response = json!({
                "type": "group_created",
                "group_id": msg["group_id"],
                "success": true,
                "error": null
            });
            sender.send(Message::Text(response.to_string())).await.ok();
        }
        Some("join_group") => {
            // Handle group joining
            // Generate Welcome message using MLS
            let response = json!({
                "type": "group_joined", 
                "group_id": msg["group_id"],
                "welcome_message": "base64_encoded_welcome",
                "error": null
            });
            sender.send(Message::Text(response.to_string())).await.ok();
        }
        _ => {}
    }
}
```

### Option 3: Use a Reference Implementation

Several open-source MLS delivery services are available:

1. **OpenMLS Delivery Service**: https://github.com/openmls/delivery-service
2. **MLS Reference Implementation**: https://github.com/mlswg/mls-implementations

## Testing Multi-Client Setup

### 1. Start the Delivery Service

```bash
# If using a reference implementation
git clone <delivery-service-repo>
cd delivery-service
cargo run
```

### 2. Run Multiple Client Instances

```bash
# Terminal 1 - Alice
cargo run --release
# Configure username: alice
# Configure delivery service: ws://localhost:8080

# Terminal 2 - Bob  
cargo run --release
# Configure username: bob
# Configure delivery service: ws://localhost:8080

# Terminal 3 - Charlie
cargo run --release
# Configure username: charlie
# Configure delivery service: ws://localhost:8080
```

### 3. Test Group Operations

#### Create a Group (Alice)
```
c: create team-chat
```

#### Join the Group (Bob and Charlie)
```
c: join <group_id_from_alice>
```

#### Send Messages
```
c: send Hello everyone!
```

#### Verify Multi-Client Communication
- Messages should appear in all clients
- Group member count should show actual members
- Messages should be end-to-end encrypted

## Troubleshooting

### Connection Issues

**Problem**: "Failed to connect to MLS Delivery Service"
- **Solution**: 
  - Verify the delivery service is running
  - Check the WebSocket URL format (ws:// or wss://)
  - Ensure firewall allows the connection
  - Test with `telnet localhost 8080` for basic connectivity

**Problem**: "Connection timeout"
- **Solution**:
  - Check if the service is listening on the correct port
  - Verify network connectivity
  - Try a different port if 8080 is blocked

### Group Joining Issues

**Problem**: "Group not found or access denied"
- **Solution**:
  - Ensure the group was created successfully
  - Check that the group ID is correct
  - Verify the delivery service is properly handling join requests
  - Check service logs for errors

**Problem**: "Failed to parse welcome message"
- **Solution**:
  - Verify the delivery service is generating valid MLS Welcome messages
  - Check that the key package format is correct
  - Ensure OpenMLS compatibility

### Message Delivery Issues

**Problem**: Messages not appearing in other clients
- **Solution**:
  - Check that all clients are connected to the same delivery service
  - Verify the delivery service is properly routing messages
  - Check for network connectivity issues
  - Review service logs for message processing errors

## Security Considerations

### TLS/SSL
- Use `wss://` URLs for production deployments
- Implement proper certificate validation
- Consider mutual TLS for client authentication

### Authentication
- Implement user authentication if required
- Validate client identities before allowing group operations
- Consider rate limiting to prevent abuse

### MLS Protocol
- Ensure proper key rotation and group rekeying
- Implement secure key storage for the delivery service
- Monitor for protocol violations or attacks

## Performance Optimization

### Scaling Considerations
- Use connection pooling for multiple clients
- Implement message queuing for high-volume scenarios
- Consider horizontal scaling with load balancing
- Monitor WebSocket connection limits

### Message Handling
- Implement efficient message routing
- Consider message persistence for offline clients
- Optimize for low-latency message delivery

## Monitoring and Logging

### Key Metrics
- Connection count and duration
- Message delivery success rate
- Group creation/join success rate
- Error rates by type

### Logging
- Log all client connections and disconnections
- Record group operations (create, join, leave)
- Track message delivery attempts
- Monitor for suspicious activity

## Next Steps

1. **Choose an implementation approach** based on your requirements
2. **Set up the delivery service** following the protocol specification
3. **Test with multiple clients** to verify functionality
4. **Implement security measures** for production use
5. **Monitor and optimize** based on usage patterns

For questions or issues, refer to the MLS protocol specification (RFC 9420) and the OpenMLS documentation. 