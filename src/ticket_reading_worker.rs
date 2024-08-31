use std::{error::Error, io};

use crate::{ data_accessors::path_provider, models::ticket::Ticket, ticket_serializer, user_input_acceptors::stdin_input_getter};

pub fn read_ticket() -> Result<Ticket, Box<dyn Error>> {
  println!("what's your ticket id?");
  let input = stdin_input_getter::get_input()?;
  get_ticket(input)
}

// this should be a trait impl
pub fn get_ticket(input: String) -> Result<Ticket, Box<dyn Error>> {
  let file_path = path_provider::get_ticket_path(&input);
  Ok(ticket_serializer::deserialize(&file_path)?)
}
