use std::error::Error;

use crate::models::ticket::Ticket;

pub trait TicketCreatingWorker {
  fn create_ticket(&self) -> Result<Ticket, Box<dyn Error>>;
}