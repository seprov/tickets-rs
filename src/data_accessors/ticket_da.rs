use std::error::Error;

use crate::models::{ticket::Ticket, ticket_id::TicketId};

pub trait TicketDa {
  fn save_ticket(&self, x: &Ticket) -> Result<(), Box<dyn Error>>;
  fn load_ticket(&self, x: &TicketId) -> Result<Ticket, Box<dyn Error>>;
}
