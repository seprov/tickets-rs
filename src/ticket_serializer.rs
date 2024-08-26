use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use crate::ticket::Ticket;

pub fn deserialize(file_path: &str) -> Result<Ticket, io::Error> {
  match File::open(file_path) {
    Ok(file) => {
      let reader = BufReader::new(file);
      let ticket: Result<Ticket, serde_json::Error> = serde_json::from_reader(reader);
      match ticket {
        Ok(t) => Ok(t),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
      }
    }
    Err(e) => Err(e),
  }
}

pub fn serialize(file_path: &str, ticket: &Ticket) {
  match File::create(file_path) {
    Ok(file) => {
      let writer = BufWriter::new(file);
      let r: Result<(), serde_json::Error> = serde_json::to_writer(writer, ticket);
    },
    Err(_) => todo!(),
  }
}
