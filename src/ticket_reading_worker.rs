use std::io;

use crate::{input_getter, ticket::Ticket, ticket_path_provider, ticket_serializer};

pub fn read_ticket() -> Result<Ticket, io::Error> {
  println!("what's your ticket id?");
  let input = input_getter::get_input()?;
  let file_path = ticket_path_provider::get_ticket_path(&input);
  ticket_serializer::deserialize(&file_path)
}
