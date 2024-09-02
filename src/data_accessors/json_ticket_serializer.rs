use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::models::ticket::Ticket;

pub fn deserialize(file_path: &str) -> Result<Ticket, Box<dyn Error>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  Ok(serde_json::from_reader::<BufReader<File>, Ticket>(reader)?)
}

pub fn serialize(file_path: &str, ticket: &Ticket) -> Result<(), Box<dyn Error>> {
  let file = File::create(file_path)?;
  let writer = BufWriter::new(file);
  let _ = serde_json::to_writer_pretty(writer, ticket)?;
  Ok(())
}
