use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::ticket::Ticket;

pub fn deserialize(file_path: &str, description_path: &str) -> Result<Ticket, io::Error> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let ticket: Result<Ticket, serde_json::Error> = serde_json::from_reader(reader);
  match ticket {
    Ok(t) => {
      let mut buf = String::new();
      let mut desc_file = File::open(description_path)?;
      let _ = desc_file.read_to_string(&mut buf)?;
      Ok(Ticket {
        description: buf,
        ..t.clone()
      })
    }
    Err(e) => Err(io::Error::from(e)),
  }
}

pub fn serialize(file_path: &str, desc_file_path: &str, ticket: &Ticket) -> Result<(), io::Error> {
  let file = File::create(file_path)?;
  let writer = BufWriter::new(file);
  let result: Result<(), serde_json::Error> = serde_json::to_writer(writer, ticket);
  match result {
    Ok(_) => {
      let mut file = File::create(desc_file_path)?;
      file.write_all(ticket.description.as_bytes())?;
      Ok(())
    },
    Err(e) => Err(io::Error::from(e)),
  }
}
