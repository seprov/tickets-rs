use std::io;

use crate::{input_getter, ticket::Ticket, ticket_serializer};

pub fn read_ticket() -> Result<Ticket, io::Error> {
  println!("what's your ticket id?");
  let input = input_getter::get_input()?;
  let file_path = format!("data/tickets/{}.json",input.trim());
  ticket_serializer::deserialize(&file_path)
}
