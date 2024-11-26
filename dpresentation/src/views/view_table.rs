use dapplication::dtos::ticket_dto::TicketDTO;
use ratatui::{
    crossterm::style::Color,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::table_colors::TableColors;

pub fn view_table(
    frame: &mut Frame,
    area: Rect,
    selected_index: Option<usize>,
    tickets: &Vec<TicketDTO>,
    table_colors: &TableColors,
    state: &mut TableState,
) {
    let header_style = Style::default()
        .fg(Color::White.into())
        .bg(Color::Blue.into());
    let header = Row::new(
        [
            "ID",
            "Level",
            "Title",
            "Status",
            "Created At",
            "Resolved At",
        ]
        .iter()
        .map(|&s| Cell::from(s)),
    )
    .style(header_style)
    .height(1);

    let rows: Vec<Row> = tickets
        .iter()
        .enumerate()
        .map(|(i, ticket)| {
            let row_style = if selected_index == Some(i) {
                Style::default()
                    .fg(table_colors.selected_row_style_fg)
                    .bg(table_colors.header_bg)
            } else if i % 2 == 0 {
                Style::default()
                    .fg(table_colors.row_fg)
                    .bg(table_colors.normal_row_color)
            } else {
                Style::default()
                    .fg(table_colors.row_fg)
                    .bg(table_colors.alt_row_color)
            };

            Row::new([
                Cell::from(ticket.id.as_str()),
                Cell::from(ticket.level.as_str()),
                Cell::from(ticket.title.as_str()),
                Cell::from(ticket.status.as_str()),
                Cell::from(ticket.created_at.to_rfc3339()),
                Cell::from(
                    ticket
                        .resolved_at
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_else(|| "".to_string()),
                ),
            ])
            .style(row_style)
        })
        .collect();

    let widths = vec![
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(30),
        Constraint::Length(15),
        Constraint::Length(25),
        Constraint::Length(25),
    ];

    frame.render_stateful_widget(
        Table::new(std::iter::once(header).chain(rows), &widths)
            .block(Block::default().borders(Borders::ALL).title("Ticket List")),
        area,
        state,
    );
}
