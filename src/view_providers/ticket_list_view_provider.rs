use crate::models::ticket::Ticket;

pub trait TicketListViewProvider{
  fn display_tickets(&self, tickets: &[Ticket]);
}