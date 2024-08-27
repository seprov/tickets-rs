use std::io;

use crate::{input_getter::get_single_char_input, ticket::Ticket};

pub fn handle_ticket(ticket: &Ticket) -> Result<Ticket,io::Error> {
  println!("okay, lets work on ticket {}", ticket.get_id_as_string());
  print!("what would you like to do?");
  print!("
  s: change schedule state
  x: save and close the ticket
  ");

  let c = get_single_char_input()?;
  match c {
    's' => todo!(),
    'x' => todo!(),
    _ => todo!()
  }
}