use std::io;

use crate::{input_getter, ticket::Ticket, path_provider, ticket_serializer};

pub fn read_ticket() -> Result<Ticket, io::Error> {
  println!("what's your ticket id?");
  let input = input_getter::get_input()?;
  let ticket_path = path_provider::get_ticket_path(&input);
  let description_path = path_provider::get_description_path(&input);
  ticket_serializer::deserialize(&ticket_path, &description_path)
}
