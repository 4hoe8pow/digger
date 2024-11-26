use crate::domain_errors::DomainError;
use crate::entites::ticket::Ticket;
use color_eyre::Result;

pub trait TicketRepository {
    fn fetch_tickets(&self) -> Result<Vec<Ticket>, DomainError>;
    fn save(&mut self, update_ticket: Ticket) -> Result<(), DomainError>;
}
