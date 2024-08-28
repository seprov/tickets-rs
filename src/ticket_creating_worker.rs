use std::{
  io::{self, ErrorKind},
  path::Path,
};

use crate::{input_getter::get_input, path_provider, ticket::Ticket, ticket_serializer};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  println!("let's create a ticket!");
  let (id_string, id_array) = get_ticket_id()?;
  let ticket = Ticket::new(id_array, "Idea".to_owned());

  let ticket_path = path_provider::get_ticket_path(&id_string);
  let description_path = path_provider::get_description_path(&id_string);

  validate_ticket_id(&id_string)?;

  println!("creating ticket");
  ticket_serializer::serialize(&ticket_path, &description_path, &ticket)?;
  println!("created ticket: {}", id_string);

  Ok(ticket)
}

fn validate_ticket_id(ticket_id: &str) -> Result<(), io::Error> {
  ticket_json_already_exists(ticket_id)?;
  // etc
  Ok(())
}

fn ticket_json_already_exists(ticket_id: &str) -> Result<(), io::Error> {
  let binding = path_provider::get_ticket_path(ticket_id);
  let path = Path::new(&binding);
  match path.exists() {
    true => Err(io::Error::new(
      ErrorKind::AlreadyExists,
      "ticket with that id already exists!",
    )),
    false => Ok(()),
  }
}

fn get_ticket_id() -> Result<(String, [u8; 8]), io::Error> {
  println!("what ticket id do you want? enter up to 8 1-byte characters");
  let binding = get_input()?;
  let input = binding.trim();
  if (&input).len() <= 8 {
    let mut buffer = [0u8; 8];
    let id_bytes = input.as_bytes();
    for (i, &b) in id_bytes.iter().enumerate().take(8) {
      buffer[i] = b;
    }
    Ok((input.to_owned(), buffer))
  } else {
    Err(io::Error::new(ErrorKind::InvalidInput, "too many chars!"))
  }
}
