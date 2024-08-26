use std::io::{self, ErrorKind, Read};

use crate::{input_getter::get_input, schedule_state::ScheduleState::Idea, ticket::Ticket, ticket_serializer};

pub fn create_ticket() -> Result<Ticket, io::Error> {
  println!("let's create a ticket!");
  println!("what ticket id do you want? enter up to 8 1-byte characters");
  let binding = get_input()?;
  let input = binding.trim();
  if (&input).len() < 8 {
    for byte in input.as_bytes() {
      println!("{}",byte)
    }
    let id_bytes = (input).as_bytes().try_into();
    match id_bytes {
      Ok(bytes) => {
        let ticket = Ticket { id: bytes, schedule_state: Idea };
        println!("creating ticket");
        let file_path = format!("data/tickets/{}.json", input);
        ticket_serializer::serialize(&file_path, &ticket);
        Ok(ticket)
      }
      Err(e) => Err(io::Error::new(ErrorKind::InvalidInput, (format!("we failed! {} {}",e,input)))),
    }
  } else {
    Err(io::Error::new(
      ErrorKind::InvalidInput,
      "wrong number of chars!",
    ))
  }
}
