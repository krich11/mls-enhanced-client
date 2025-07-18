# MLS Enhanced Client - Usage Guide

## Quick Start

1. **Build and Run**:
   ```bash
   cargo build --release
   cargo run
   ```

2. **First Time Setup**:
   - The app will create a default `config.json` file
   - Use the `settings` command to configure your username and delivery service address
   - Default delivery service: `127.0.0.1:8080`

## Interface Overview

The TUI consists of three main areas:

### 1. Sidebar (Left)
- **Groups**: Shows all MLS groups you're part of
- **Navigation**: Use ↑/↓ arrows to select groups
- **Active Group**: Highlighted group receives messages

### 2. Message Area (Center)
- **Messages**: Shows conversation history for the active group
- **Timestamps**: Each message shows when it was sent
- **Scrolling**: Use Page Up/Down to scroll through history

### 3. Input Area (Bottom)
- **Command Mode**: Type commands starting with `/`
- **Message Mode**: Type messages to send to active group
- **Status Bar**: Shows current status and errors

## Commands

### Basic Commands
- `create <group_name>` - Create a new MLS group
- `join <group_id>` - Join an existing group (requires invitation)
- `send <message>` - Send a message to the active group
- `help` - Show command help
- `settings` - Open settings screen
- `quit` - Exit the application

### Examples
```
create team-chat
join abc123def456
send Hello, everyone!
settings
quit
```

## Settings Screen

Access via `settings` command:

1. **Username**: Your identity in MLS groups
2. **Delivery Service**: Address of the MLS Delivery Service
3. **Save**: Press Enter to save changes
4. **Cancel**: Press Escape to cancel

## Keyboard Shortcuts

### General Navigation
- `Tab` - Switch between input modes
- `Escape` - Return to normal mode
- `Enter` - Execute command or send message
- `↑/↓` - Navigate groups in sidebar
- `Page Up/Down` - Scroll messages

### Input Modes
- **Normal Mode**: Navigate interface
- **Command Mode**: Type commands (starts with `/`)
- **Message Mode**: Type messages to send
- **Settings Mode**: Edit configuration

## Multi-Group Messaging

### Creating Groups
1. Use `create <group_name>` to create a new group
2. You become the group administrator
3. Share the group ID with others to invite them

### Joining Groups
1. Get a group ID from an existing member
2. Use `join <group_id>` to join
3. You'll receive the group's message history

### Switching Between Groups
1. Use arrow keys to select different groups in sidebar
2. Active group is highlighted
3. Messages are sent to the active group only

## Troubleshooting

### Common Issues

#### 1. **Connection Failed**
```
Error: Failed to connect to delivery service
```
**Solution**:
- Check if MLS Delivery Service is running on `127.0.0.1:8080`
- Verify network connectivity
- Update delivery service address in settings

#### 2. **Invalid Group ID**
```
Error: Group not found
```
**Solution**:
- Verify the group ID is correct
- Ensure you have permission to join the group
- Check with the group administrator

#### 3. **KeyPackage Issues**
```
Error: Failed to create KeyPackage
```
**Solution**:
- Restart the application
- Check username is set correctly in settings
- Verify OpenMLS crypto provider is working

#### 4. **Configuration Problems**
```
Error: Failed to load config
```
**Solution**:
- Delete `config.json` to reset to defaults
- Check file permissions
- Verify JSON syntax if manually edited

#### 5. **TUI Display Issues**
```
Interface appears corrupted
```
**Solution**:
- Resize terminal window
- Ensure terminal supports Unicode
- Try different terminal emulator

### Network Issues

#### Delivery Service Not Running
1. Clone and run the MLS Delivery Service:
   ```bash
   git clone https://github.com/krich11/mls-delivery-service
   cd mls-delivery-service
   cargo run
   ```

2. Or use Docker:
   ```bash
   docker run -p 8080:8080 mls-delivery-service
   ```

#### Firewall Blocking Connection
- Ensure port 8080 is open
- Check firewall rules
- Try different port in settings

### Performance Issues

#### High Memory Usage
- Restart application periodically
- Limit number of active groups
- Clear message history if needed

#### Slow Message Delivery
- Check network latency
- Verify delivery service performance
- Consider using local delivery service

## Advanced Usage

### Configuration File
Edit `config.json` manually:
```json
{
  "username": "your-username",
  "delivery_service_address": "127.0.0.1:8080"
}
```

### Multiple Instances
- Each instance needs a unique username
- Can join same groups from different instances
- Useful for testing multi-user scenarios

### Development Testing
1. Start delivery service
2. Run multiple client instances:
   ```bash
   # Terminal 1
   cargo run
   
   # Terminal 2  
   cargo run
   
   # Terminal 3
   cargo run
   ```
3. Create groups and test messaging

## Security Considerations

### Cryptographic Agility
- Uses OpenMLS 0.7 with Ed25519 signatures
- AES-128-GCM for message encryption
- X25519 for key exchange
- Designed for future KEM upgrades

### Key Management
- Keys stored in memory only
- No persistent key storage
- Restart clears all cryptographic state

### Network Security
- End-to-end encryption via MLS
- Delivery service sees only encrypted messages
- No plaintext message storage

## Logging and Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug cargo run
```

### Common Log Messages
- `INFO`: Normal operation
- `WARN`: Non-critical issues
- `ERROR`: Critical failures
- `DEBUG`: Detailed operation info

## Getting Help

### In-Application Help
- Use `help` command for quick reference
- Press `F1` for context-sensitive help

### External Resources
- [OpenMLS Documentation](https://openmls.tech/)
- [MLS RFC 9420](https://tools.ietf.org/rfc/rfc9420.txt)
- [Ratatui Documentation](https://ratatui.rs/)

### Reporting Issues
1. Check this troubleshooting guide
2. Enable debug logging
3. Reproduce the issue
4. Report with logs and steps to reproduce