use std::io::{self};

use crate::{
  data_access::path_provider, model::ticket::Ticket, ticket_id_validator,
  ticket_serializer, user_input::stdin_ticket_id_getter,
};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  // stdout specific view for creating ticket
  println!("please enter a ticket id for your new ticket");
  // stdin specific user input for getting ticket id
  let (id_string, id_array) = stdin_ticket_id_getter::get_ticket_id()?;

  ticket_id_validator::ticket_json_already_exists(&id_string)?;

  let ticket = Ticket::new(id_array, "Idea".to_owned());

  let file_path = path_provider::get_ticket_path(&id_string);

  println!("creating ticket");
  ticket_serializer::serialize(&file_path, &ticket)?;
  println!("created ticket: {}", id_string);

  Ok(ticket)
}
