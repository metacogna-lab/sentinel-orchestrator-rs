// Sentinel Orchestrator CLI
// A beautiful, interactive CLI tool for managing and interacting with the Sentinel backend

mod api;
mod app;
mod modes;
mod types;
mod ui;

use crate::app::{handle_chat_message, AppState};
use crate::api::ApiClient;
use crate::modes::Mode;
use crate::ui::*;
use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Sentinel Orchestrator CLI
#[derive(Parser, Debug)]
#[command(name = "sentinel-cli")]
#[command(about = "Interactive CLI for Sentinel Orchestrator", long_about = None)]
struct Args {
    /// Backend API base URL
    #[arg(short, long, default_value = "http://localhost:3000")]
    url: String,

    /// API key for authentication (or set SENTINEL_API_KEY env var)
    #[arg(short = 'k', long)]
    api_key: Option<String>,
}

/// Main application
struct App {
    state: Arc<RwLock<AppState>>,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl App {
    fn new(state: Arc<RwLock<AppState>>) -> Result<Self> {
        enable_raw_mode().context("Failed to enable raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .context("Failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).context("Failed to create terminal")?;

        Ok(Self { state, terminal })
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            self.draw().await?;

            if crossterm::event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if self.handle_key(key.code).await? {
                            break;
                        }
                    }
                }
            }

            let state = self.state.read().await;
            if state.should_exit {
                break;
            }
        }

        Ok(())
    }

    async fn draw(&mut self) -> Result<()> {
        let state = self.state.read().await;

        self.terminal.draw(|f| {
            match state.mode {
                Mode::MainMenu => {
                    render_main_menu(f, state.menu_selection);
                }
                Mode::Chat => {
                    render_chat(f, &state.messages, &state.input);
                }
                Mode::Investigation => {
                    render_investigation(f, &state.input, &state.investigation_results);
                }
                Mode::Debugging => {
                    render_debugging(f, &state.debug_logs);
                }
                Mode::SystemStatus => {
                    render_system_status(f, &state.health);
                }
            }

            if let Some(error) = &state.error {
                render_error(f, error);
            }
        })?;

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyCode) -> Result<bool> {
        let mut state = self.state.write().await;

        match key {
            KeyCode::Char('q') => {
                state.should_exit = true;
                return Ok(true);
            }
            KeyCode::Esc => {
                if state.mode != Mode::MainMenu {
                    state.mode = Mode::MainMenu;
                    state.input.clear();
                } else {
                    state.should_exit = true;
                    return Ok(true);
                }
            }
            KeyCode::Tab => {
                // Cycle through modes
                state.mode = match state.mode {
                    Mode::MainMenu => Mode::Chat,
                    Mode::Chat => Mode::Investigation,
                    Mode::Investigation => Mode::Debugging,
                    Mode::Debugging => Mode::SystemStatus,
                    Mode::SystemStatus => Mode::MainMenu,
                };
                state.input.clear();
            }
            KeyCode::Up => {
                match state.mode {
                    Mode::MainMenu => {
                        if state.menu_selection > 0 {
                            state.menu_selection -= 1;
                        } else {
                            state.menu_selection = 4; // Wrap to last item
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match state.mode {
                    Mode::MainMenu => {
                        if state.menu_selection < 4 {
                            state.menu_selection += 1;
                        } else {
                            state.menu_selection = 0; // Wrap to first item
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Enter => {
                match state.mode {
                    Mode::MainMenu => {
                        // Handle menu selection
                        match state.menu_selection {
                            0 => state.mode = Mode::Chat,
                            1 => state.mode = Mode::Investigation,
                            2 => state.mode = Mode::Debugging,
                            3 => state.mode = Mode::SystemStatus,
                            4 => {
                                state.should_exit = true;
                                return Ok(true);
                            }
                            _ => {}
                        }
                    }
                    Mode::Chat => {
                        // Send chat message
                        if !state.input.trim().is_empty() {
                            let message = state.input.clone();
                            state.input.clear();
                            
                            // Handle chat message with streaming
                            if let Err(e) = handle_chat_message(&mut *state, message).await {
                                state.set_error(format!("Failed to send message: {}", e));
                            }
                        }
                    }
                    Mode::SystemStatus => {
                        // Refresh health status
                        if let Err(e) = state.update_health().await {
                            state.set_error(format!("Failed to update health: {}", e));
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                if state.mode == Mode::Chat || state.mode == Mode::Investigation {
                    state.input.pop();
                }
            }
            KeyCode::Char(c) => {
                if state.mode == Mode::Chat || state.mode == Mode::Investigation {
                    state.input.push(c);
                }
            }
            _ => {}
        }

        Ok(false)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Get API key from argument or environment variable
    let api_key = args.api_key.or_else(|| std::env::var("SENTINEL_API_KEY").ok());

    // Initialize API client
    let api_client = Arc::new(if let Some(key) = api_key {
        ApiClient::with_api_key(args.url, key).context("Failed to create API client")?
    } else {
        ApiClient::new(args.url).context("Failed to create API client")?
    });

    // Initialize app state
    let state = Arc::new(RwLock::new(AppState::new(api_client)));

    // Create and run app
    let mut app = App::new(state).context("Failed to create app")?;
    app.run().await.context("Failed to run app")?;

    Ok(())
}

