use std::{error::Error, fs};

use crate::models::ticket::Ticket;

use super::json_ticket_serializer;

pub fn list_tickets() -> Result<Vec<Ticket>, Box<dyn Error>> {
  let path = "data/tickets/";
  let mut vec = vec![];

  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();

    if let Some(filename) = path.file_stem() {
      if let Some(filename_str) = filename.to_str() {
        let file_path = format!("data/tickets/{}.json", filename_str);
        let ticket = json_ticket_serializer::deserialize(&file_path)?;
        vec.push(ticket);
      }
    }
  }
  Ok(vec)
}
