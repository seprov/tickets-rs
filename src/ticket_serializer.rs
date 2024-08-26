use std::fs::File;
use std::io::{self, BufReader};

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
