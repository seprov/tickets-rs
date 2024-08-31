use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use crate::model::ticket::Ticket;

pub fn deserialize(file_path: &str) -> Result<Ticket, io::Error> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let ticket: Result<Ticket, serde_json::Error> = serde_json::from_reader(reader);
  match ticket {
    Ok(t) => Ok(t),
    Err(e) => Err(io::Error::from(e)),
  }
}

pub fn serialize(file_path: &str, ticket: &Ticket) -> Result<(), io::Error> {
  let file = File::create(file_path)?;
  let writer = BufWriter::new(file);
  let result: Result<(), serde_json::Error> = serde_json::to_writer_pretty(writer, ticket);
  match result {
    Ok(_) => Ok(()),
    Err(e) => Err(io::Error::from(e)),
  }
}
