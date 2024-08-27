use std::{collections::HashMap, io};

use crate::{input_getter::get_single_char_input, schedule_state_provider, ticket::Ticket};

pub fn handle_ticket(ticket: Ticket) -> Result<Ticket, io::Error> {
  println!("okay, lets work on ticket {}", ticket.get_id_as_string());
  print!("what would you like to do?");
  print!(
    "
  s: change schedule state
  x: save and close the ticket
  "
  );

  let c = get_single_char_input()?;
  match c {
    's' => change_schedule_state(&ticket),
    'x' => Ok(ticket),
    _ => Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("you entered {}, which is not valid!", c),
    )),
  }
}

fn change_schedule_state(ticket: &Ticket) -> Result<Ticket, io::Error> {
  println!("your ticket's schedule state is currently {}", ticket.schedule_state);
  println!("what schedule state do you want?");
  let schedule_states = schedule_state_provider::get_schedule_states();
  
  todo!()
}

fn get_character_schedule_state_mapping(schedule_states: &[String]) -> HashMap<char,String> {
  todo!()
}
