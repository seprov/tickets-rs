use std::error::Error;

use crate::models::ticket::Ticket;

pub trait TicketListProvider {
  fn get_ticket_list(&self) -> Result<Vec<Ticket>, Box<dyn Error>>;
}
