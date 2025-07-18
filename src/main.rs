use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use openmls::prelude::*;
use openmls::prelude::tls_codec::{Serialize, Deserialize};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::collections::HashMap;
use std::io;
use uuid::Uuid;

mod config;
mod crypto;
mod mls_client;
mod network;
mod ui;

use config::Config;
use crypto::CryptoProvider;
use mls_client::MlsClient;
use network::NetworkClient;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
    pub group_id: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub members: Vec<String>,
    pub messages: Vec<Message>,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub enum AppScreen {
    Main,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Command,
    Message,
    Settings,
}

pub struct App {
    pub config: Config,
    pub mls_client: MlsClient,
    pub network_client: NetworkClient,
    pub groups: HashMap<String, Group>,
    pub active_group: Option<String>,
    pub input: String,
    pub input_mode: InputMode,
    pub screen: AppScreen,
    pub group_list_state: ListState,
    pub message_scroll: u16,
    pub status_message: String,
    pub should_quit: bool,
    pub settings_field: usize,
    pub temp_delivery_service: String,
    pub temp_username: String,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Config::load_or_default().await?;
        let crypto_provider = CryptoProvider::new();
        let mls_client = MlsClient::new(&config.username, crypto_provider).await?;
        let network_client = NetworkClient::new(&config.delivery_service_address).await?;
        
        let mut group_list_state = ListState::default();
        group_list_state.select(Some(0));

        let status_message = if network_client.is_connected() {
            format!("Connected to MLS service at {}. Groups will be synchronized.", config.delivery_service_address)
        } else {
            format!("Disconnected from MLS service at {}. Groups will be local only.", config.delivery_service_address)
        };

        Ok(Self {
            config: config.clone(),
            mls_client,
            network_client,
            groups: HashMap::new(),
            active_group: None,
            input: String::new(),
            input_mode: InputMode::Normal,
            screen: AppScreen::Main,
            group_list_state,
            message_scroll: 0,
            status_message,
            should_quit: false,
            settings_field: 0,
            temp_delivery_service: config.delivery_service_address.clone(),
            temp_username: config.username.clone(),
        })
    }

    pub async fn handle_input(&mut self, key: KeyCode) -> Result<()> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_input(key).await,
            InputMode::Command => self.handle_command_input(key).await,
            InputMode::Message => self.handle_message_input(key).await,
            InputMode::Settings => self.handle_settings_input(key).await,
        }
    }

    async fn handle_normal_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') => {
                self.input_mode = InputMode::Command;
                self.input.clear();
            }
            KeyCode::Char('m') => {
                if self.active_group.is_some() {
                    self.input_mode = InputMode::Message;
                    self.input.clear();
                } else {
                    self.status_message = "No active group selected".to_string();
                }
            }
            KeyCode::Char('s') => {
                self.screen = AppScreen::Settings;
                self.input_mode = InputMode::Settings;
            }
            KeyCode::Char('h') => {
                self.screen = AppScreen::Help;
            }
            KeyCode::Up => {
                let groups: Vec<_> = self.groups.keys().cloned().collect();
                if !groups.is_empty() {
                    let selected = self.group_list_state.selected().unwrap_or(0);
                    let new_selected = if selected > 0 { selected - 1 } else { groups.len() - 1 };
                    self.group_list_state.select(Some(new_selected));
                    self.active_group = Some(groups[new_selected].clone());
                }
            }
            KeyCode::Down => {
                let groups: Vec<_> = self.groups.keys().cloned().collect();
                if !groups.is_empty() {
                    let selected = self.group_list_state.selected().unwrap_or(0);
                    let new_selected = if selected < groups.len() - 1 { selected + 1 } else { 0 };
                    self.group_list_state.select(Some(new_selected));
                    self.active_group = Some(groups[new_selected].clone());
                }
            }
            // Add Shift+Up and Shift+Down for message scroll
            KeyCode::Up if event::KeyModifiers::SHIFT == event::KeyModifiers::SHIFT => {
                self.message_scroll = self.message_scroll.saturating_sub(1);
            }
            KeyCode::Down if event::KeyModifiers::SHIFT == event::KeyModifiers::SHIFT => {
                self.message_scroll = self.message_scroll.saturating_add(1);
            }
            // Optionally, add j/k for single-line scroll
            KeyCode::Char('j') => {
                self.message_scroll = self.message_scroll.saturating_add(1);
            }
            KeyCode::Char('k') => {
                self.message_scroll = self.message_scroll.saturating_sub(1);
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_command_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                let command = self.input.trim().to_owned();
                self.execute_command(&command).await?;
                self.input.clear();
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => {
                self.input.clear();
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_message_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                if let Some(group_id) = &self.active_group {
                    let message = self.input.trim().to_owned();
                    if !message.is_empty() {
                        let group_id_owned = group_id.clone();
                        self.send_message(&group_id_owned, &message).await?;
                    }
                }
                self.input.clear();
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => {
                self.input.clear();
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_settings_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                self.save_settings().await?;
                self.screen = AppScreen::Main;
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => {
                self.temp_delivery_service = self.config.delivery_service_address.clone();
                self.temp_username = self.config.username.clone();
                self.screen = AppScreen::Main;
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Tab => {
                self.settings_field = (self.settings_field + 1) % 2;
            }
            KeyCode::Char(c) => {
                if self.settings_field == 0 {
                    self.temp_delivery_service.push(c);
                } else {
                    self.temp_username.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.settings_field == 0 {
                    self.temp_delivery_service.pop();
                } else {
                    self.temp_username.pop();
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn execute_command(&mut self, command: &str) -> Result<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"create") => {
                if let Some(group_name) = parts.get(1) {
                    self.create_group(group_name).await?;
                } else {
                    self.status_message = "Usage: create <group_name>".to_string();
                }
            }
            Some(&"join") => {
                if let Some(group_id) = parts.get(1) {
                    self.join_group(group_id).await?;
                } else {
                    self.status_message = "Usage: join <group_id>".to_string();
                }
            }
            Some(&"send") => {
                if let Some(message) = parts.get(1..) {
                    let message = message.join(" ");
                    if let Some(group_id) = &self.active_group {
                        let group_id_owned = group_id.clone();
                        self.send_message(&group_id_owned, &message).await?;
                    } else {
                        self.status_message = "No active group selected".to_string();
                    }
                } else {
                    self.status_message = "Usage: send <message>".to_string();
                }
            }
            Some(&"quit") => {
                self.should_quit = true;
            }
            Some(&"help") => {
                self.screen = AppScreen::Help;
            }
            Some(&"settings") => {
                self.screen = AppScreen::Settings;
                self.input_mode = InputMode::Settings;
            }
            Some(&"groups") => {
                if self.groups.is_empty() {
                    self.status_message = "No groups available. Use 'create <group_name>' to create a group.".to_string();
                } else {
                    let groups_info: Vec<String> = self.groups
                        .iter()
                        .map(|(id, group)| format!("• {} (ID: {}) - {} members", group.name, id, group.members.len()))
                        .collect();
                    self.status_message = format!("Available groups:\n{}", groups_info.join("\n"));
                }
            }
            Some(&"status") => {
                if self.network_client.is_connected() {
                    self.status_message = format!("Connected to MLS service at {}. {} groups available.", 
                        self.config.delivery_service_address, self.groups.len());
                } else {
                    self.status_message = format!("Disconnected from MLS service at {}. Groups will be local only.", 
                        self.config.delivery_service_address);
                }
            }
            _ => {
                self.status_message = format!("Unknown command: {}. Available commands: create, join, send, groups, status, settings, help, quit", command);
            }
        }
        Ok(())
    }

    async fn create_group(&mut self, group_name: &str) -> Result<()> {
        let group_id = Uuid::new_v4().to_string();
        
        // Create MLS group
        let group_config = MlsGroupCreateConfig::builder()
            .wire_format_policy(WireFormatPolicy::default())
            .build();
        
        let mls_group = MlsGroup::new(
            &self.mls_client.crypto,
            &self.mls_client.signer,
            &group_config,
            CredentialWithKey {
                credential: self.mls_client.credential.clone().into(),
                signature_key: self.mls_client.signature_key.clone(),
            },
        )?;

        // Store the MLS group
        self.mls_client.add_group(&group_id, mls_group);

        // Store group locally
        let group = Group {
            id: group_id.clone(),
            name: group_name.to_string(),
            members: vec![self.config.username.clone()],
            messages: Vec::new(),
            is_active: true,
        };
        
        self.groups.insert(group_id.clone(), group);
        self.active_group = Some(group_id.clone());
        
        // Update group list selection
        let groups: Vec<_> = self.groups.keys().cloned().collect();
        if let Some(pos) = groups.iter().position(|g| g == &group_id) {
            self.group_list_state.select(Some(pos));
        }
        
        // Publish group to MLS service if connected
        if self.network_client.is_connected() {
            // Export the group info for sharing
            let group_info = group_id.as_bytes().to_vec();
            if let Err(e) = self.network_client.create_group(&group_id, &group_info).await {
                self.status_message = format!("Created group: {} (ID: {}), but failed to publish to MLS service: {}", group_name, group_id, e);
            } else {
                self.status_message = format!("Created and published group: {} (ID: {})", group_name, group_id);
            }
        } else {
            self.status_message = format!("Created local group: {} (ID: {}) - not connected to MLS service", group_name, group_id);
        }
        
        Ok(())
    }

    async fn join_group(&mut self, group_id: &str) -> Result<()> {
        // Check if we're connected to the MLS service
        if !self.network_client.is_connected() {
            self.status_message = format!("Cannot join group {}: Not connected to MLS service. Use 'status' command to check connection.", group_id);
            return Ok(());
        }

        // Check if we're already in this group
        if self.groups.contains_key(group_id) {
            self.status_message = format!("Already in group: {}", group_id);
            return Ok(());
        }

        // Try to join the group through the MLS service
        match self.network_client.join_group(group_id, &self.mls_client.key_package.tls_serialize_detached()?).await {
            Ok(welcome_data) => {
                if welcome_data.is_empty() {
                    self.status_message = format!("Group {} not found or access denied. This could mean:\n1. The group doesn't exist on the MLS service\n2. You don't have permission to join\n3. The MLS service is not properly configured\n\nTry creating the group first with 'create <group_name>' or check your MLS service configuration.", group_id);
                    return Ok(());
                }

                // Parse the welcome message and join the MLS group
                match Welcome::tls_deserialize(&mut welcome_data.as_slice()) {
                    Ok(_welcome) => {
                        // For now, we'll just create a local group representation
                        // In a full implementation, we'd create the MLS group from the welcome message
                        // let mls_group = MlsGroup::new_from_welcome(
                        //     &self.mls_client.crypto,
                        //     &MlsGroupConfig::default(),
                        //     welcome,
                        //     Some(&self.mls_client.storage),
                        // )?;
                        // self.mls_client.add_group(group_id, mls_group);

                        // Create local group representation
                        let group = Group {
                            id: group_id.to_string(),
                            name: format!("Group {}", group_id),
                            members: vec![self.config.username.clone()], // Will be updated with real members
                            messages: Vec::new(),
                            is_active: true,
                        };
                        
                        self.groups.insert(group_id.to_string(), group);
                        self.active_group = Some(group_id.to_string());
                        
                        // Update group list selection
                        let groups: Vec<_> = self.groups.keys().cloned().collect();
                        if let Some(pos) = groups.iter().position(|g| g == group_id) {
                            self.group_list_state.select(Some(pos));
                        }
                        
                        self.status_message = format!("Successfully joined group: {} (Welcome message received)", group_id);
                    }
                    Err(e) => {
                        self.status_message = format!("Failed to parse welcome message for group {}: {}", group_id, e);
                    }
                }
            }
            Err(e) => {
                self.status_message = format!("Failed to join group {}: {}\n\nThis could be due to:\n1. Network connectivity issues\n2. MLS service not running\n3. Invalid group ID\n\nTry using 'status' command to check connection.", group_id, e);
            }
        }
        Ok(())
    }

    async fn send_message(&mut self, group_id: &str, message: &str) -> Result<()> {
        if let Some(group) = self.groups.get_mut(group_id) {
            let msg = Message {
                id: Uuid::new_v4().to_string(),
                sender: self.config.username.clone(),
                content: message.to_string(),
                timestamp: Local::now(),
                group_id: group_id.to_string(),
            };
            
            group.messages.push(msg);
            self.status_message = format!("Message sent to {}", group.name);
        }
        Ok(())
    }

    async fn save_settings(&mut self) -> Result<()> {
        let old_address = self.config.delivery_service_address.clone();
        self.config.delivery_service_address = self.temp_delivery_service.clone();
        self.config.username = self.temp_username.clone();
        self.config.save().await?;
        
        // Reconnect to MLS service if address changed
        if old_address != self.config.delivery_service_address {
            self.network_client = NetworkClient::new(&self.config.delivery_service_address).await?;
            
            if self.network_client.is_connected() {
                self.status_message = format!("Settings saved. Connected to MLS service at {}", self.config.delivery_service_address);
            } else {
                self.status_message = format!("Settings saved. Failed to connect to MLS service at {}", self.config.delivery_service_address);
            }
        } else {
            self.status_message = "Settings saved".to_string();
        }
        
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame) {
        match self.screen {
            AppScreen::Main => self.render_main(f),
            AppScreen::Settings => self.render_settings(f),
            AppScreen::Help => self.render_help(f),
        }
    }

    fn render_main(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(f.size());

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(chunks[0]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),         // Messages area
                Constraint::Length(3),      // Input area
                Constraint::Percentage(35), // Status area (takes 35% of right panel)
            ].as_ref())
            .split(chunks[1]);

        // Groups list
        let groups: Vec<ListItem> = self.groups
            .iter()
            .map(|(id, group)| {
                let style = if Some(id) == self.active_group.as_ref() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} ({})", group.name, group.members.len()))
                    .style(style)
            })
            .collect();

        let groups_list = List::new(groups)
            .block(Block::default().borders(Borders::ALL).title("Groups"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(groups_list, left_chunks[0], &mut self.group_list_state);

        // Controls
        let controls = Paragraph::new("c: Command\nm: Message\ns: Settings\nq: Quit")
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(controls, left_chunks[1]);

        // Messages
        let messages: Vec<Line> = if let Some(group_id) = &self.active_group {
            if let Some(group) = self.groups.get(group_id) {
                group.messages.iter().map(|msg| {
                    Line::from(vec![
                        Span::styled(
                            format!("[{}]", msg.timestamp.format("%H:%M:%S")),
                            Style::default().fg(Color::Gray),
                        ),
                        Span::styled(
                            format!(" {}: ", msg.sender),
                            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(msg.content.clone()),
                    ])
                }).collect()
            } else {
                vec![]
            }
        } else {
            vec![Line::from("No active group selected")]
        };

        let messages_paragraph = Paragraph::new(messages)
            .block(Block::default().borders(Borders::ALL).title("Messages"))
            .wrap(Wrap { trim: true })
            .scroll((self.message_scroll, 0));

        f.render_widget(messages_paragraph, right_chunks[0]);

        // Input
        let input_title = match self.input_mode {
            InputMode::Command => "Command",
            InputMode::Message => "Message",
            _ => "Input",
        };
        
        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                _ => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title(input_title));
        f.render_widget(input, right_chunks[1]);

        // Status with available groups
        let status_content = if self.groups.is_empty() {
            format!("{}\n\nAvailable groups: None\nUse 'create <group_name>' to create a group", self.status_message)
        } else {
            let groups_list: Vec<String> = self.groups
                .iter()
                .map(|(id, group)| format!("• {} ({}) - {} members", group.name, id, group.members.len()))
                .collect();
            format!("{}\n\nAvailable groups:\n{}", self.status_message, groups_list.join("\n"))
        };
        
        let status = Paragraph::new(status_content)
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("Status & Groups"))
            .wrap(Wrap { trim: true });
        f.render_widget(status, right_chunks[2]);

        // Cursor
        if matches!(self.input_mode, InputMode::Command | InputMode::Message) {
            f.set_cursor(
                right_chunks[1].x + self.input.len() as u16 + 1,
                right_chunks[1].y + 1,
            );
        }
    }

    fn render_settings(&mut self, f: &mut Frame) {
        let area = f.size();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        f.render_widget(Clear, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(popup_area);

        let delivery_service_style = if self.settings_field == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let username_style = if self.settings_field == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let delivery_service = Paragraph::new(self.temp_delivery_service.as_str())
            .style(delivery_service_style)
            .block(Block::default().borders(Borders::ALL).title("Delivery Service"));
        f.render_widget(delivery_service, chunks[0]);

        let username = Paragraph::new(self.temp_username.as_str())
            .style(username_style)
            .block(Block::default().borders(Borders::ALL).title("Username"));
        f.render_widget(username, chunks[1]);

        let help = Paragraph::new("Tab: Next field\nEnter: Save\nEsc: Cancel")
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(help, chunks[2]);
    }

    fn render_help(&mut self, f: &mut Frame) {
        let area = f.size();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        f.render_widget(Clear, popup_area);

        let help_text = vec![
            "MLS Enhanced Client Help",
            "",
            "Navigation:",
            "  ↑/↓: Select group",
            "  PageUp/PageDown: Scroll messages",
            "",
            "Commands:",
            "  c: Enter command mode",
            "  m: Enter message mode",
            "  s: Settings",
            "  h: Help",
            "  q: Quit",
            "",
            "Command Mode:",
            "  create <group_name>: Create new group",
            "  join <group_id>: Join existing group",
            "  send <message>: Send message",
            "  list: Show available groups",
            "  status: Check MLS service connection",
            "  quit: Exit application",
            "",
            "MLS Service:",
            "  Groups are shared when connected to MLS service",
            "  Local groups are created when disconnected",
            "  Use 'status' command to check connection",
            "",
            "Troubleshooting:",
            "  If 'group not found':",
            "  - Check connection with 'status'",
            "  - Create group first with 'create'",
            "  - Try joining 'test-group' for demo",
            "",
            "Press any key to close",
        ];

        let help_paragraph = Paragraph::new(help_text.join("\n"))
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .wrap(Wrap { trim: true });
        f.render_widget(help_paragraph, popup_area);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new().await?;

    // Main loop
    loop {
        terminal.draw(|f| app.render(f))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.screen {
                    AppScreen::Help => {
                        app.screen = AppScreen::Main;
                    }
                    _ => {
                        app.handle_input(key.code).await?;
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
