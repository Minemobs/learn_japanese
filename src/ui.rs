use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let h_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(10),
                Constraint::Percentage(45),
            ]
            .as_ref(),
        )
        .split(f.size());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(15),
                Constraint::Percentage(5),
                Constraint::Percentage(20),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default()
        .title("Kana")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    let rect = Rect::new(
        h_chunk[1].x,
        chunks[1].y,
        h_chunk[1].width,
        chunks[1].height,
    );
    let paragraph = Paragraph::new(app.current_hiragana().unwrap().get_hiragana().to_string())
        .alignment(Alignment::Center)
        .block(block);
    f.render_widget(paragraph, rect);
    let block = Block::default()
        .title(" Roumanji (latin char) ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let rect = Rect::new(
        h_chunk[1].x - 10,
        chunks[3].y,
        h_chunk[1].width + 20,
        chunks[3].height,
    );
    let paragraph = Paragraph::new(app.get_input())
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, rect);
    if let Some(popup_content) = app.get_popup() {
        let block = Block::default()
            .title("Popup")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center);
        let text = Paragraph::new(popup_content.to_string())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(block);
        let area = centered_rect(15, 20, f.size());
        f.render_widget(Clear, area);
        f.render_widget(text, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
