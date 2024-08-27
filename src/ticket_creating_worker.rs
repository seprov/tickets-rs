use std::io::{self, ErrorKind};

use crate::{
  input_getter::get_input, schedule_state::ScheduleState::Idea, ticket::Ticket, ticket_serializer,
};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  println!("let's create a ticket!");
  println!("what ticket id do you want? enter up to 8 1-byte characters");
  let binding = get_input()?;
  let input = binding.trim();
  if (&input).len() <= 8 {
    let mut buffer = [0u8; 8];
    let id_bytes = input.as_bytes();
    for (i, &b) in id_bytes.iter().enumerate().take(8) {
      buffer[8 - (i + 1)] = b;
    }

    let ticket = Ticket {
      id: buffer,
      schedule_state: Idea,
    };

    println!("creating ticket");
    let file_path = format!("data/tickets/{}.json", input);
    ticket_serializer::serialize(&file_path, &ticket);

    println!("created ticket: {}", input);

    Ok(ticket)
  } else {
    Err(io::Error::new(ErrorKind::InvalidInput, "too many chars!"))
  }
}
