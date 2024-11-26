use dapplication::dtos::ticket_dto::TicketDTO;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn view_ticket_detail(frame: &mut Frame, area: Rect, selected_ticket: TicketDTO) {
    // レイアウト分割: 上部にタイトル、下部に詳細入力エリアを作成
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 上部: タイトル
            Constraint::Min(5),    // 下部: 詳細入力エリア
        ])
        .split(area);

    // [id] タイトル形式の表示
    let title_text = format!("[{}] {}", selected_ticket.id, selected_ticket.title);
    let title_paragraph = Paragraph::new(Span::from(Span::styled(
        title_text,
        Style::default().fg(Color::Yellow), // タイトルを黄色で強調
    )))
    .block(Block::default().borders(Borders::ALL).title("Title"));

    // 詳細入力エリア (TextAreaにデータを反映)
    let detail_paragraph = Paragraph::new(selected_ticket.completion_condition).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Completion Condition"),
    );

    frame.render_widget(title_paragraph, chunks[0]);
    frame.render_widget(detail_paragraph, chunks[1]);
}
