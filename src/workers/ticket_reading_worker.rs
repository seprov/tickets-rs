use std::error::Error;

use crate::models::ticket::Ticket;

pub trait TicketReadingWorker {
  fn read_ticket(&self) -> Result<Ticket, Box<dyn Error>>;
}
