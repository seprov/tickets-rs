use std::error::Error;

use crate::{
  data_accessors::path_provider,
  models::{ticket::Ticket, ticket_id::TicketId},
};

use super::{json_ticket_serializer, ticket_da::TicketDa};

// currently using json implementation

pub struct JsonTicketDa;

impl TicketDa for JsonTicketDa {
   fn save_ticket(&self, x: &Ticket) -> Result<(), Box<dyn Error>> {
    let file_path = path_provider::get_ticket_path(&x.id);
    json_ticket_serializer::serialize(&file_path, x)
  }

  fn load_ticket(&self, x: &TicketId) -> Result<Ticket, Box<dyn Error>> {
    let file_path = path_provider::get_ticket_path(x);
    json_ticket_serializer::deserialize(&file_path)
  }
}
