use std::io::{self, ErrorKind};

use crate::{input_getter::get_input, ticket::Ticket, ticket_serializer};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  println!("let's create a ticket!");
  let (id_string, id_array) = get_ticket_id()?;
  let ticket = Ticket::new(id_array, "Idea".to_owned());

  println!("creating ticket");
  let file_path = format!("data/tickets/{}.json", id_string);
  ticket_serializer::serialize(&file_path, &ticket);

  println!("created ticket: {}", id_string);

  Ok(ticket)
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
