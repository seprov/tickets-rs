use std::{
  io::{self, ErrorKind},
  path::Path,
};

use crate::{input_getter, path_provider};

pub fn get_ticket_id() -> Result<(String, [u8; 8]), io::Error> {
  println!("enter up to 8 1-byte characters");
  let binding = input_getter::get_input()?;
  let input = binding.trim();
  if (&input).len() <= 8 {
    let mut buffer = [0u8; 8];
    let id_bytes = input.as_bytes();
    for (i, &b) in id_bytes.iter().enumerate().take(8) {
      buffer[i] = b;
    }
    validate_ticket_id(input)?;
    Ok((input.to_owned(), buffer))
  } else {
    Err(io::Error::new(ErrorKind::InvalidInput, "too many chars!"))
  }
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
