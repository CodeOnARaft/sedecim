use std::{io, thread, time::{Duration, Instant}, sync::mpsc};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction,Rect},
    Terminal
};

 
use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment };
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{ BorderType,  Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};
 

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture,Event as CEvent , KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::env;
use std::fs::File;
use std::io::Read;

enum Event<I> {
    Input(I),
    Tick,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("You must pass a file name");
        return Ok(());
    }

    const BUFFER_SIZE: usize = 256;
    let mut file = File::open(&args[1])?;
    let mut buffer = [0;BUFFER_SIZE];

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
    enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Errors");

    loop {
        terminal.draw(|f| {
            let size = f.size();

             // Vertical layout
             let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),                    
                ]
                .as_ref(),
            )
            .split(size);
              // Title
            let title = draw_title();
            f.render_widget(title, chunks[0]);

            let mut lines:Vec<String> = vec![];
            for i in 0..4 {
                lines.push(format!("{:02x} {:02x} {:02x} {:02x} {:02x} | {}{}{}{}{} "
                , buffer[(i*5)], buffer[(i*5)+1], buffer[(i*5)+2], buffer[(i*5)+3], buffer[(i*5)+4]
                , buffer[(i*5)] as char, buffer[(i*5)+1] as char, buffer[(i*5)+2] as char, buffer[(i*5)+3] as char, buffer[(i*5)+4] as char))
            }        
            
           let para = 

                Paragraph::new(vec![
                    Spans::from(Span::raw(&lines[0])),
                    Spans::from(Span::raw(&lines[1])),
                    Spans::from(Span::raw(&lines[2])),
                    Spans::from(Span::raw(&lines[3])),
                    Spans::from(Span::raw(format!("{} x {}",size.width,size.height))),
                    
                ])                
                .alignment(Alignment::Left)
                .block(  Block::default()
                .title("Block")
                .borders(Borders::ALL));
            f.render_widget(para, chunks[1]);
            
            let status = draw_status();
            f.render_widget(status, chunks[2]);

        }).expect("Issues");

        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode();
                    terminal.show_cursor().expect("Errors");
                    terminal.clear();
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
    Paragraph::new("Test TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

pub fn draw_status<'a>() -> Paragraph<'a> {
    Paragraph::new("Status: ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}