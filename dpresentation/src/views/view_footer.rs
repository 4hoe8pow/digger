use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn view_footer(frame: &mut Frame, area: Rect, mode: String) {
    let footer_text = format!(
        "Mode: {} | (q) Exit | (k) Up | (j) Down | (l) Inquery Mode",
        mode
    );

    frame.render_widget(
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Control Guide"),
            ),
        area,
    );
}
