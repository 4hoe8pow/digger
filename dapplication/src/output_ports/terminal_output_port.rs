use crate::dtos::ticket_dto::TicketDTO;
use ratatui::{layout::Rect, Frame};
use tui_textarea::TextArea;

pub trait TerminalOutputPort {
    fn draw_table(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    );
    fn draw_ticket_detail(&self, frame: &mut Frame, area: Rect, selected_ticket: TicketDTO);
    fn draw_ticket_form(
        &self,
        frame: &mut Frame,
        area: Rect,
        text_field: &mut TextArea,
        selected_ticket: TicketDTO,
    );

    fn draw_footer(&self, frame: &mut Frame, area: Rect, mode: String);

    fn next_row(&mut self, items_len: usize);
    fn previous_row(&mut self, items_len: usize);
    fn selected_index(&self) -> Option<usize>;
    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect);
    fn notify(&self, message: String);
}
