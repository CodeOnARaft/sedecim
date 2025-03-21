use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BorderType, Paragraph};

use super::{AppMode, sedecim_file_page::SedecimFilePage};

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
            let mut page = app.file_info.get_page(curr_byte);
            let mut page_line = (curr_byte - page.page_start) / byte_count;
            for i in 0..20 {
                if curr_byte > app.file_info.file_size {
                    continue;
                }

                let page_number = SedecimFilePage::get_page(curr_byte);
                if page.page_id != page_number {
                    page = app.file_info.get_page(curr_byte);
                    page_line = 0;
                }
                

                let mut curr_str = format!(" {:06x}  ", curr_byte);
                let mut char_str = format!(" ");
                for indx in 0..byte_count {
                    let ii = ((page_line * byte_count) + indx)   as usize;                    
                    match app.mode {
                        AppMode::Standard
                            if app.selected_line as u64 == i
                                && app.selected_value as u64 == indx =>
                        {
                            curr_str.push_str(&format!("!|{:02x}!| ", page.buffer[ii]));

                            if page.buffer[ii] >= 32 && page.buffer[ii].is_ascii() {
                                char_str.push_str(&format!("!|{}!| ", page.buffer[ii] as char));
                            } else {
                                char_str.push_str("!|.!| ");
                            }
                        }

                        _ => {
                            curr_str.push_str(&format!("{:02x} ", page.buffer[ii]));
                            if page.buffer[ii] >= 32 && page.buffer[ii].is_ascii() {
                                char_str.push_str(&format!("{} ", page.buffer[ii] as char));
                            } else {
                                char_str.push_str(". ");
                            }
                        }
                    }
                }

                page_line += 1;
                lines.push(format!("{} | {}", curr_str, char_str));
                curr_byte += byte_count;
            }

            let mut spans: Vec<Spans> = vec![];
            for l in 0..lines.len() {
                match app.mode {
                    AppMode::Standard if app.selected_line == (l as i32) => {
                        let str_split: Vec<&str> = lines[l].split("!|").collect();

                        let nsp = Spans::from(vec![
                            Span::styled(str_split[0], Style::default().fg(Color::White)),
                            Span::styled(
                                str_split[1],
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::RAPID_BLINK)
                                    .add_modifier(Modifier::BOLD)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(str_split[2], Style::default().fg(Color::White)),
                            Span::styled(
                                str_split[3],
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::RAPID_BLINK)
                                    .add_modifier(Modifier::BOLD)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(str_split[4], Style::default().fg(Color::White)),
                        ]);
                        spans.push(nsp);
                    }

                    _ => {
                        let new_span = Spans::from(Span::raw(&lines[l]));
                        spans.push(new_span);
                    }
                }
            }

            match app.mode {
                AppMode::Jump => {
                    let s = format!("Jump to Address (HEX): {}", app.jump_value);
                    spans.push(Spans::from(Span::raw("".to_owned())));
                    let mut newspns = Spans::from(vec![
                        Span::styled(s, Style::default().fg(Color::White)),
                        Span::styled(
                            " ",
                            Style::default()
                                .add_modifier(Modifier::RAPID_BLINK)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                    ]);

                    spans.push(newspns);

                    newspns = Spans::from(vec![Span::styled(
                        app.error.clone(),
                        Style::default().fg(Color::Red),
                    )]);
                    spans.push(newspns);
                }

                _ => {}
            }

            let para = Paragraph::new(spans).alignment(Alignment::Left).block(
                Block::default()
                    .title(format!(
                        " {} ({}, {:06x}) ",
                        &app.file_info.file_name,
                        &app.file_info.file_size,
                        &app.file_info.file_size
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
