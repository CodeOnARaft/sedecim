mod events;
mod sedecim_file_info;
mod ui;

use std::io::{self, Stdout};
use tui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    cursor::{EnableBlinking, MoveTo, Show as ShowCursor},
    event::{EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

pub struct App {
    events: events::SecdecimEvents,
    pub file_info: sedecim_file_info::sedecim_file_info,
    pub selected_line: i32,
    pub selected_value: i32,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let events = events::SecdecimEvents::new();
        let file_info = sedecim_file_info::sedecim_file_info::new(String::from(&args[1]));
        let selected_line = 0;
        let selected_value = 0;
        Self {
            events,
            file_info,
            selected_line,
            selected_value,
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
        Terminal::new(backend).expect("Errors")
    }

    pub fn run(&mut self) {
        let mut terminal = self.init();
        match self.runner(&mut terminal) {
            _ =>{
                let _ = disable_raw_mode();
                terminal.show_cursor().expect("Errors");
                let _ = terminal.clear();

                println!("Thank you for using secedim");
            }
        }
    }

    fn runner(&mut self,terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {        

        loop {
            let _ = ui::draw_ui(self, terminal);

            match self.events.next() {
                events::Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {                       
                        break;
                    }

                    KeyCode::Up => {
                        self.selected_line -= 1;
                        if self.selected_line <= 0 {
                            self.selected_line = 0;
                            self.file_info
                                .scroll(sedecim_file_info::move_values::up_line);
                        }
                    }

                    KeyCode::Down => {
                        self.selected_line += 1;
                        if self.selected_line >= 19 {
                            self.selected_line = 19;
                            self.file_info
                                .scroll(sedecim_file_info::move_values::down_line);
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
                        self.file_info
                            .scroll(sedecim_file_info::move_values::up_page);
                    }

                    KeyCode::PageDown => {
                        self.file_info
                            .scroll(sedecim_file_info::move_values::down_page);
                    }

                    _ => {}
                },
                events::Event::Tick => {}
            }
        }

        Ok(())
    }
}
