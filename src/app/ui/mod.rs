use std::io::Stdout;

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

use super::App;

pub fn draw_ui(
    app: &mut super::App,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal
        .draw(|f| {
            let size = f.size();

            // Vertical layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                .split(size);
            // Title
            let title = draw_title();

            f.render_widget(title, chunks[0]);

            let byte_count: u64 = 10;
            let mut lines: Vec<String> = vec![];
            let mut curr_byte = app.file_info.file_offset;
            for i in 0..20 {
                if curr_byte > app.file_info.file_size {
                    continue;
                }

                let mut curr_str = format!(" {:06x}  ", curr_byte);
                let mut char_str = format!(" ");
                for indx in 0..byte_count {
                    let ii = ((i * byte_count) + indx) as usize;
                    curr_str.push_str(&format!("{:02x} ", app.file_info.buffer[ii]));
                    if app.file_info.buffer[ii] >= 32 {
                        char_str.push_str(&format!("{} ", app.file_info.buffer[ii] as char));
                    } else {
                        char_str.push_str(". ");
                    }
                }

                lines.push(format!("{} | {}", curr_str, char_str));
                curr_byte += byte_count;
            }

            let mut spans: Vec<Spans> = vec![];
            for l in 0..lines.len() {
                if app.selected_line == (l as i32) {
                    let split: usize = (app.selected_value * 3) as usize;
                    let (first, next) = lines[l].split_at(9 + split);
                    let (styled, rest) = next.split_at(2);

                    let rest_length = rest.len() - (((9 - app.selected_value) * 2) as usize);
                    let (left_char, rest_char) = rest.split_at(rest_length);
                    let (styled_char, right_char) = rest_char.split_at(1);

                    let nsp = Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::White)),
                        Span::styled(
                            styled,
                            Style::default()
                                .fg(Color::LightYellow)
                                .add_modifier(Modifier::SLOW_BLINK)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(left_char, Style::default().fg(Color::White)),
                        Span::styled(
                            styled_char,
                            Style::default()
                                .fg(Color::LightYellow)
                                .add_modifier(Modifier::SLOW_BLINK)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(right_char, Style::default().fg(Color::White)),
                    ]);
                    spans.push(nsp);
                } else {
                    let new_span = Spans::from(Span::raw(&lines[l]));
                    spans.push(new_span);
                }
            }
            let para = Paragraph::new(spans).alignment(Alignment::Left).block(
                Block::default()
                    .title(format!(
                        " {} ({}) ",
                        &app.file_info.file_name, &app.file_info.file_size
                    ))
                    .borders(Borders::ALL),
            );
            f.render_widget(para, chunks[1]);
        })
        .expect("Issues");

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
