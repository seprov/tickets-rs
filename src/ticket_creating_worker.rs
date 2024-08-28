use std::io::{self};

use crate::{path_provider, ticket::Ticket, ticket_id_getter, ticket_id_validator, ticket_serializer};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  println!("please enter a ticket id for your new ticket");
  let (id_string, id_array) = ticket_id_getter::get_ticket_id()?;

  ticket_id_validator::ticket_json_already_exists(&id_string)?;

  let ticket = Ticket::new(id_array, "Idea".to_owned());
  
  let file_path = path_provider::get_ticket_path(&id_string);

  println!("creating ticket");
  ticket_serializer::serialize(&file_path, &ticket)?;
  println!("created ticket: {}", id_string);

  Ok(ticket)
}
