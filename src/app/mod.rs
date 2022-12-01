use std::{
    io,
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

use symbols::line;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BorderType, Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};

use crossterm::{
    cursor::{
        DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, Show as ShowCursor,
    },
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::env;
use std::fs::File;
use std::io::Read;

enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 1 {
            println!("You must pass a file name");
            return Ok(());
        }

        const BUFFER_SIZE: usize = 256;
        let file_name = String::from(&args[1]);
        let mut file = File::open(&file_name)?;
        let mut buffer = [0; BUFFER_SIZE];
        let file_size = std::fs::metadata(&file_name)?.len();
        let _ = file.by_ref().take(256).read(&mut buffer)?;

        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move || -> ! {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

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
        let mut terminal = Terminal::new(backend).expect("Errors");

        loop {
            terminal
                .draw(|f| {
                    let size = f.size();

                    // Vertical layout
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3),
                                Constraint::Min(10)
                            ]
                            .as_ref(),
                        )
                        .split(size);
                    // Title
                    let title = App::draw_title();
                    f.render_widget(title, chunks[0]);

                    let byte_count: u64 = 10;
                    let mut lines: Vec<String> = vec![];
                    let mut curr_byte = 0;
                    for i in 0..20 {
                        if curr_byte > file_size {
                            continue;
                        }

                        let mut curr_str = format!(" {:06x}  ", curr_byte);
                        let mut char_str = format!(" ");
                        for indx in 0..byte_count {
                            let ii = ((i * byte_count) + indx) as usize;
                            curr_str.push_str(&format!("{:02x} ", buffer[ii]));
                            char_str.push_str(&format!("{} ", buffer[ii] as char));
                        }

                        lines.push(format!("{} | {}", curr_str, char_str));
                        curr_byte += byte_count;
                    }

                    let mut spans: Vec<Spans> = vec![];
                    for l in 0..lines.len() {
                        let mut newSpan = Spans::from(Span::raw(&lines[l]));

                        spans.push(newSpan);
                    }
                    let para = Paragraph::new(spans).alignment(Alignment::Left).block(
                        Block::default()
                            .title(format!(" {} ({}) ", &file_name, &file_size))
                            .borders(Borders::ALL),
                    );
                    f.render_widget(para, chunks[1]);

                   
                })
                .expect("Issues");

            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        let _ = disable_raw_mode();
                        terminal.show_cursor().expect("Errors");
                        let _ = terminal.clear();
                        break;
                    }

                    _ => {}
                },
                Event::Tick => {}
            }
        }

        Ok(())
    }

    pub fn draw_title<'a>() -> Paragraph<'a> {
        Paragraph::new("sedecim")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            )
    }

   
}
