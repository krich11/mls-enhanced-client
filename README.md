# MLS Enhanced Client

A fully functional, CLI-based MLS (Messaging Layer Security) messaging client built in Rust with a modern Terminal User Interface (TUI). This client supports multiple simultaneous MLS groups, group creation, and secure message sending/receiving with end-to-end encryption.

## Features

- **Multiple MLS Groups**: Support for simultaneous participation in multiple MLS groups
- **Modern TUI Interface**: Teams/Slack-like experience with `ratatui` featuring:
  - Sidebar for group selection
  - Message display area with timestamps
  - Interactive input fields
  - Settings screen
  - Help screen
- **End-to-End Encryption**: Uses OpenMLS 0.5 with `openmls_rust_crypto` for secure messaging
- **Flexible Configuration**: JSON-based configuration with delivery service settings
- **Async Networking**: Built with Tokio for efficient TCP networking
- **Cryptographic Agility**: Designed for future key encapsulation mechanisms (KEMs)

## Installation

### Prerequisites

- Rust 1.70+ (2021 edition)
- Linux environment
- Access to MLS Delivery Service (default: `127.0.0.1:8080`)

### Building from Source

```bash
git clone https://github.com/krich11/mls-enhanced-client.git
cd mls-enhanced-client
cargo build --release
```

### Running the Client

```bash
cargo run
```

## Usage

### Interface Overview

The client features a split-screen interface:

- **Left Panel**: 
  - Groups list (top)
  - Controls help (bottom)
- **Right Panel**:
  - Message display (top)
  - Input field (middle)
  - Status bar (bottom)

### Navigation

- **Arrow Keys (↑/↓)**: Select active group
- **PageUp/PageDown**: Scroll through messages
- **c**: Enter command mode
- **m**: Enter message mode (when group is selected)
- **s**: Open settings screen
- **h**: Show help screen
- **q**: Quit application

### Commands

Enter command mode by pressing `c`, then type:

- `create <group_name>` - Create a new MLS group
- `join <group_id>` - Join an existing group
- `send <message>` - Send a message to the active group
- `settings` - Open settings screen
- `help` - Show help screen
- `quit` - Exit the application

### Settings Configuration

Access settings by pressing `s` or using the `settings` command. Configure:

- **Delivery Service Address**: IP:PORT of the MLS Delivery Service (default: `127.0.0.1:8080`)
- **Username**: Your identity for MLS credentials (default: `user`)

Settings are automatically saved to `config.json`.

### Message Sending

1. Select a group using arrow keys
2. Press `m` to enter message mode
3. Type your message
4. Press Enter to send
5. Press Esc to cancel

### Group Management

#### Creating a Group

```bash
# In command mode (press 'c')
create my-team
```

#### Joining a Group

```bash
# In command mode (press 'c')
join <group-id>
```

## Configuration

The client uses a `config.json` file for settings:

```json
{
  "username": "your-username",
  "delivery_service_address": "127.0.0.1:8080"
}
```

Configuration is automatically created on first run and can be modified through the settings screen.

## Architecture

### Core Components

- **MLS Client**: Handles MLS cryptographic operations using OpenMLS
- **Network Client**: Manages TCP connections to the MLS Delivery Service
- **TUI Interface**: Provides the terminal-based user interface
- **Configuration Manager**: Handles settings persistence

### Cryptographic Details

- **Ciphersuite**: `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519`
- **Protocol Version**: MLS 1.0
- **Credentials**: BasicCredential with username-based identity
- **Key Exchange**: X25519 DHKEM
- **Signature**: Ed25519

## Troubleshooting

### Common Issues

#### 1. Failed to Connect to Delivery Service

**Symptoms**: "Connection timeout to MLS Delivery Service" or "Failed to connect"

**Solutions**:
- Ensure the MLS Delivery Service is running at the configured address
- Check network connectivity
- Verify the delivery service address in settings (`s` key)
- Default service should be running on `127.0.0.1:8080`

#### 2. Configuration File Issues

**Symptoms**: Settings not persisting or JSON parsing errors

**Solutions**:
- Delete `config.json` to regenerate with defaults
- Ensure proper JSON formatting
- Check file permissions

#### 3. Group Creation/Join Issues

**Symptoms**: Groups not appearing or cryptographic errors

**Solutions**:
- Verify MLS Delivery Service is properly configured
- Check username configuration for valid identity
- Ensure network connectivity to delivery service

#### 4. TUI Display Issues

**Symptoms**: Interface not rendering properly or garbled text

**Solutions**:
- Ensure terminal supports Unicode characters
- Try resizing terminal window
- Check terminal compatibility with crossterm

#### 5. Compilation Errors

**Symptoms**: Build failures with OpenMLS dependencies

**Solutions**:
- Update Rust to latest stable version
- Clear cargo cache: `cargo clean`
- Update dependencies: `cargo update`

### Debug Mode

For verbose logging, set the environment variable:

```bash
RUST_LOG=debug cargo run
```

### Network Debugging

To test network connectivity:

```bash
telnet 127.0.0.1 8080
```

## Development

### Project Structure

```
src/
├── main.rs           # Main application and TUI logic
├── config.rs         # Configuration management
├── crypto.rs         # Cryptographic provider
├── mls_client.rs     # MLS operations
├── network.rs        # Network client
└── ui.rs            # UI utilities
```

### Adding New Features

1. **New Commands**: Add to `execute_command()` in `main.rs`
2. **UI Changes**: Modify render functions in `main.rs`
3. **Network Protocol**: Update `network.rs` for new message types
4. **Cryptographic Features**: Extend `mls_client.rs`

### Testing

Run tests with:

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run lints
cargo clippy

# Security audit
cargo audit
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenMLS](https://github.com/openmls/openmls) for MLS implementation
- [ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
- [Tokio](https://github.com/tokio-rs/tokio) for async runtime

## Support

For issues and questions:
- Create an issue on GitHub
- Check the troubleshooting section above
- Review the OpenMLS documentation for MLS-specific questions
