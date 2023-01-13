mod events;
mod sedecim_file_info;
mod ui;

use std::{
    io::{self, Stdout},
};
use tui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    cursor::{EnableBlinking, MoveTo, Show as ShowCursor},
    event::{EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

pub enum AppMode {
    Standard,
    Jump,
}

pub struct App {
    events: events::SecdecimEvents,
    pub file_info: sedecim_file_info::SedecimFileInfo,
    pub selected_line: i32,
    pub selected_value: i32,
    pub mode: AppMode,
    pub jump_value: String,
    pub error: String,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let events = events::SecdecimEvents::new();
        let file_info = sedecim_file_info::SedecimFileInfo::new(String::from(&args[1]));
        let selected_line = 0;
        let selected_value = 0;
        let mode = AppMode::Standard;
        let jump_value: String = String::default();
        let error: String = String::default();
        Self {
            events,
            file_info,
            selected_line,
            selected_value,
            mode,
            jump_value,
            error,
        }
    }

    fn init(&mut self) -> Terminal<CrosstermBackend<Stdout>> {
        self.file_info.read_bytes();

        // setup terminal
        let _ = enable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            ShowCursor,
            EnableBlinking,
            MoveTo(10, 25)
        );

        let backend = CrosstermBackend::new(stdout);
        Terminal::new(backend).expect("Error creating a new Terminal in App Init")
    }

    pub fn run(&mut self) {
        let mut terminal = self.init();
        match self.runner(&mut terminal) {
            _ => {
                let _ = disable_raw_mode();
                terminal.show_cursor().expect("Errors");
                let _ = terminal.clear();

                println!("\n\n");
                println!("                _              _            ");
                println!(" ___   ___   __| |  ___   ___ (_) _ __ ___  ");
                println!("/ __| / _ \\ / _` | / _ \\ / __|| || '_ ` _ \\ ");
                println!("\\__ \\|  __/| (_| ||  __/| (__ | || | | | | |");
                println!("|___/ \\___| \\__,_| \\___| \\___||_||_| |_| |_|");

                println!("\n\nThank you for using sedecim!\n\n");
            }
        }
    }

    fn runner(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let _ = ui::draw_ui(self, terminal);

            if self.handle_input() {
                break;
            }
        }

        Ok(())
    }

    fn handle_input(&mut self) -> bool {
        let mut quit = false;
        match self.mode {
            AppMode::Standard => quit = self.handle_input_standard(),
            AppMode::Jump => self.handle_input_jump(),
            _ => {}
        }

        return quit;
    }

    fn handle_input_standard(&mut self) -> bool {
        match self.events.next() {
            events::Event::Input(event) => match event.code {
                KeyCode::Char('g') => match event.modifiers {
                    KeyModifiers::CONTROL => {
                        self.mode = AppMode::Jump;
                        self.jump_value = String::default();
                    }
                    _ => {}
                },

                KeyCode::Char('q') => {
                    return true;
                }

                KeyCode::Up => {
                    self.selected_line -= 1;
                    if self.selected_line <= 0 {
                        self.selected_line = 0;
                        self.file_info.scroll(sedecim_file_info::MoveValues::UpLine);
                    }
                }

                KeyCode::Down => {
                    self.selected_line += 1;
                    if self.selected_line >= 19 {
                        self.selected_line = 19;
                        self.file_info
                            .scroll(sedecim_file_info::MoveValues::DownLine);
                    }
                }

                KeyCode::Right => {
                    self.selected_value += 1;
                    if self.selected_value > 9 {
                        self.selected_value = 0;
                    }
                }

                KeyCode::Left => {
                    self.selected_value -= 1;
                    if self.selected_value < 0 {
                        self.selected_value = 9;
                    }
                }
                KeyCode::PageUp => {
                    self.file_info.scroll(sedecim_file_info::MoveValues::UpPage);
                }

                KeyCode::PageDown => {
                    self.file_info
                        .scroll(sedecim_file_info::MoveValues::DownPage);
                }

                KeyCode::Esc => match self.mode {
                    AppMode::Jump => {
                        self.mode = AppMode::Standard;
                    }
                    _ => {}
                },

                _ => {}
            },
            events::Event::Tick => {}
        }

        false
    }

    fn handle_input_jump(&mut self) {
        match self.events.next() {
            events::Event::Input(event) => match event.code {
                KeyCode::Esc => self.mode = AppMode::Standard,

                KeyCode::Char(chr) =>{
                    match "0123456789abcdef".chars().position(|c| c == chr) {
                        Some(_) => {
                            self.jump_value.push(chr);
                        }
                        None =>{}
                    }

                    
                }
                
                KeyCode::Backspace => {
                    let _ = self.jump_value.pop();
                }

                KeyCode::Enter => {
                    let address = i64::from_str_radix(&self.jump_value, 16).unwrap_or(-1);
                    let max_address = self.file_info.file_size as i64;

                    if address >= 0 && address <= max_address {
                        let offset = address % 10;
                        let address = address - offset;
                        self.file_info.file_offset = address as u64;
                        self.selected_value = offset as i32;
                        self.selected_line = 0;
                        self.file_info.read_bytes();
                        self.mode = AppMode::Standard;    
                    }
                }
                _ => {}
            },

            events::Event::Tick => {}
        }
    }
}
