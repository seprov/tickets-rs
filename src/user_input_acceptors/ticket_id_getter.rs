use std::error::Error;

use crate::models::ticket_id::TicketId;

pub trait TicketIdGetter{
  fn get_ticket_id(&self) -> Result<TicketId,Box<dyn Error>>;
}