use std::error::Error;

use crate::models::ticket::Ticket;

pub trait TicketCreatingWorker {
  fn create_ticket() -> Result<Ticket, Box<dyn Error>>;
}
