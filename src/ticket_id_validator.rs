use std::{io::{self, ErrorKind}, path::Path};

use crate::path_provider;

pub fn ticket_json_already_exists(ticket_id: &str) -> Result<(), io::Error> {
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
