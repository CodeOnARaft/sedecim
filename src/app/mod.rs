mod events;
mod sedecim_file_info;
mod ui;

use std::{
    io::{self, Stdout},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BorderType, Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};

use crossterm::{
    cursor::{
        DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, Show as ShowCursor,
    },
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

pub struct App {
    events: events::SecdecimEvents,
    pub file_info: sedecim_file_info::sedecim_file_info,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let events = events::SecdecimEvents::new();
        let file_info = sedecim_file_info::sedecim_file_info::new(String::from(&args[1]));

        Self { events, file_info }
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

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {

       let mut terminal = self.init();       

        loop {
            terminal
                .draw(|f| {
                    let size = f.size();

                    // Vertical layout
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                        .split(size);
                    // Title
                    let title = ui::draw_title();

                    f.render_widget(title, chunks[0]);

                    let byte_count: u64 = 10;
                    let mut lines: Vec<String> = vec![];
                    let mut curr_byte = self.file_info.file_offset;
                    for i in 0..20 {
                        if curr_byte > self.file_info.file_size {
                            continue;
                        }

                        let mut curr_str = format!(" {:06x}  ", curr_byte);
                        let mut char_str = format!(" ");
                        for indx in 0..byte_count {
                            let ii = ((i * byte_count) + indx) as usize;
                            curr_str.push_str(&format!("{:02x} ", self.file_info.buffer[ii]));
                            char_str.push_str(&format!("{} ", self.file_info.buffer[ii] as char));
                        }

                        lines.push(format!("{} | {}", curr_str, char_str));
                        curr_byte += byte_count;
                    }

                    let mut spans: Vec<Spans> = vec![];
                    for l in 0..lines.len() {
                        let new_span = Spans::from(Span::raw(&lines[l]));

                        spans.push(new_span);
                    }
                    let para = Paragraph::new(spans).alignment(Alignment::Left).block(
                        Block::default()
                            .title(format!(
                                " {} ({}) ",
                                &self.file_info.file_name, &self.file_info.file_size
                            ))
                            .borders(Borders::ALL),
                    );
                    f.render_widget(para, chunks[1]);
                })
                .expect("Issues");

            match self.events.next() {
                events::Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        let _ = disable_raw_mode();
                        terminal.show_cursor().expect("Errors");
                        let _ = terminal.clear();
                        break;
                    }

                    KeyCode::Char('j') | KeyCode::Up => {
                        if self.file_info.file_offset >= 10 {
                            self.file_info.file_offset -= 10;
                            self.file_info.read_bytes();
                        } else {
                            self.file_info.file_offset = 0;
                        }
                    }

                    KeyCode::Char('k') | KeyCode::Down => {
                        if self.file_info.file_offset <= self.file_info.file_size - 10 {
                            self.file_info.file_offset += 10;
                            self.file_info.read_bytes();
                        }
                    }

                    KeyCode::PageUp => {
                        if self.file_info.file_offset >= sedecim_file_info::BUFFER_SIZE_u64 {
                            self.file_info.file_offset -= sedecim_file_info::BUFFER_SIZE_u64;
                            self.file_info.read_bytes();
                        } else {
                            self.file_info.file_offset = 0;
                        }
                    }

                    KeyCode::PageDown => {
                        if self.file_info.file_offset
                            <= self.file_info.file_size - sedecim_file_info::BUFFER_SIZE_u64
                        {
                            self.file_info.file_offset += sedecim_file_info::BUFFER_SIZE_u64;
                            self.file_info.read_bytes();
                        }
                    }

                    _ => {}
                },
                events::Event::Tick => {}
            }
        }

        Ok(())
    }
}
