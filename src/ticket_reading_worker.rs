use std::error::Error;

use crate::{ data_accessors::path_provider, models::{ticket::Ticket, ticket_id::TicketId}, ticket_serializer, user_input_acceptors::stdin_ticket_id_getter};


// create struct and trait impl
pub fn read_ticket() -> Result<Ticket, Box<dyn Error>> {
  println!("what's your ticket id?");
  let input = stdin_ticket_id_getter::get_ticket_id()?;
  get_ticket(input)
}

// create struct and trait impl
pub fn get_ticket(input: TicketId) -> Result<Ticket, Box<dyn Error>> {
  let file_path = path_provider::get_ticket_path(&input);
  Ok(ticket_serializer::deserialize(&file_path)?)
}
