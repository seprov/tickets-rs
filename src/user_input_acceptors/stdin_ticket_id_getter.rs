use std::io::{self, ErrorKind};

use crate::{models::ticket_id::TicketId, user_input_acceptors::stdin_input_getter};

pub fn get_ticket_id() -> Result<TicketId, io::Error> {
  println!("enter up to 8 1-byte characters");
  let binding = stdin_input_getter::get_input()?;
  let input = binding.trim();
  if (&input).len() <= 8 {
    let mut buffer = [0u8; 8];
    let id_bytes = input.as_bytes();
    for (i, &b) in id_bytes.iter().enumerate().take(8) {
      buffer[i] = b;
    }

    Ok(TicketId::from(buffer))
  } else {
    Err(io::Error::new(ErrorKind::InvalidInput, "too many chars!"))
  }
}
