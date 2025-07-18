# MLS Enhanced Client

A Terminal User Interface (TUI) client for the Messaging Layer Security (MLS) protocol, providing secure group messaging capabilities.

## Features

- **Secure Group Messaging**: Create and join MLS-protected groups
- **Real-time Communication**: Connect to MLS delivery service for shared groups
- **Local Mode**: Work offline with local groups when service is unavailable
- **Terminal Interface**: Full-featured TUI with keyboard navigation
- **Connection Status**: Real-time feedback on MLS service connectivity

## Installation

### Prerequisites

- Rust 1.70+ and Cargo
- Network access to MLS delivery service (optional for local mode)

### Build and Run

```bash
# Clone the repository
git clone <repository-url>
cd mls-enhanced-client

# Build the project
cargo build --release

# Run the client
cargo run --release
```

## Usage

### Starting the Application

1. Run the application: `cargo run --release`
2. The client will attempt to connect to the MLS delivery service
3. Check connection status in the bottom status bar

### Navigation

- **↑/↓**: Navigate between groups
- **PageUp/PageDown**: Scroll through messages
- **c**: Enter command mode
- **m**: Enter message mode (when group is selected)
- **s**: Open settings
- **h**: Show help
- **q**: Quit application

### Commands

#### Command Mode (`c` key)

- `create <group_name>`: Create a new group
- `join <group_id>`: Join an existing group
- `send <message>`: Send a message to the active group
- `status`: Check MLS service connection status
- `settings`: Open settings screen
- `help`: Show help screen
- `quit`: Exit application

#### Message Mode (`m` key)

- Type your message and press Enter to send
- Press Esc to cancel

### Settings

Access settings with the `s` key or `settings` command:

- **Delivery Service Address**: URL/address of the MLS delivery service
- **Username**: Your identity for MLS groups

Press Tab to navigate between fields, Enter to save, Esc to cancel.

## Configuration

The client stores configuration in `config.json`:

```json
{
  "username": "your_username",
  "delivery_service_address": "127.0.0.1:8080"
}
```

## Troubleshooting

### Connection Issues

**Problem**: "Disconnected from MLS service" status
- **Cause**: MLS delivery service is not running or unreachable
- **Solution**: 
  - Verify the delivery service is running
  - Check the service address in settings
  - Ensure network connectivity
  - Use `status` command to test connection

**Problem**: Cannot join groups
- **Cause**: Not connected to MLS service
- **Solution**: 
  - Connect to MLS delivery service first
  - Check service address in settings
  - Verify group ID exists on the service

### Group Issues

**Problem**: Groups show "(1)" member count
- **Cause**: Working in local mode or group not properly synced
- **Solution**:
  - Connect to MLS service for shared groups
  - Use `status` command to verify connection
  - Rejoin groups after connecting to service

**Problem**: Cannot create groups
- **Cause**: MLS service connection issues
- **Solution**:
  - Check connection status
  - Verify service address
  - Groups can be created locally when disconnected

### Application Issues

**Problem**: Terminal display issues
- **Cause**: Terminal compatibility or size issues
- **Solution**:
  - Ensure terminal supports UTF-8
  - Resize terminal window if needed
  - Use a modern terminal emulator

**Problem**: Key bindings not working
- **Cause**: Terminal configuration or conflicts
- **Solution**:
  - Check terminal key mapping
  - Ensure no other applications are capturing keys
  - Try different terminal emulator

### Performance Issues

**Problem**: Slow response or high CPU usage
- **Cause**: Large message history or network issues
- **Solution**:
  - Restart application to clear message cache
  - Check network connectivity
  - Reduce message history if needed

## Development

### Project Structure

```
src/
├── main.rs          # Main application logic and TUI
├── config.rs        # Configuration management
├── crypto.rs        # Cryptographic utilities
├── mls_client.rs    # MLS protocol client
├── network.rs       # Network communication
└── ui.rs           # UI components (if any)
```

### Building for Development

```bash
# Debug build
cargo build

# Run with debug output
RUST_LOG=debug cargo run

# Run tests
cargo test
```

### Adding Features

1. **New Commands**: Add to `execute_command()` in `main.rs`
2. **Network Features**: Extend `NetworkClient` in `network.rs`
3. **MLS Features**: Extend `MlsClient` in `mls_client.rs`
4. **UI Changes**: Modify rendering functions in `main.rs`

## Security Considerations

- **Key Management**: MLS keys are stored in memory only
- **Network Security**: Ensure TLS/encryption for delivery service
- **Authentication**: Verify delivery service authenticity
- **Group Access**: Control who can join your groups

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license information here]

## Support

For issues and questions:
- Check this README for troubleshooting
- Review the help screen in the application (`h` key)
- Open an issue on the project repository
