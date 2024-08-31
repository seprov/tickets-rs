use std::io;

use crate::{ data_access::path_provider, model::ticket::Ticket, ticket_serializer, user_input::stdin_input_getter};

pub fn read_ticket() -> Result<Ticket, io::Error> {
  println!("what's your ticket id?");
  let input = stdin_input_getter::get_input()?;
  get_ticket(input)
}

// this should be a trait impl
pub fn get_ticket(input: String) -> Result<Ticket, io::Error> {
  let file_path = path_provider::get_ticket_path(&input);
  ticket_serializer::deserialize(&file_path)
}
