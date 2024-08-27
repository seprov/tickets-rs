use std::{
  collections::HashMap,
  fs::{remove_file, File},
  io::{self, Read, Write},
  process::Command,
};

use crate::{input_getter, path_provider, schedule_state_provider, ticket::Ticket};

pub fn handle_ticket(ticket: &Ticket) -> Result<Ticket, io::Error> {
  println!("okay, lets work on ticket {}", ticket.get_id_as_string());
  print!("what would you like to do?");
  print!(
    "
  s: change schedule state
  d: add description
  p: point
  x: save and close the ticket
  "
  );

  let c = input_getter::get_single_char_input()?;
  match c {
    's' => change_schedule_state(&ticket),
    'd' => change_description(&ticket),
    'p' => change_estimate(&ticket),
    'x' => Ok(ticket.clone()),
    _ => Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("you entered {}, which is not valid!", c),
    )),
  }
}

fn change_estimate(ticket: &Ticket) -> Result<Ticket, io::Error> {
  if let Some(current_estimate) = ticket.estimate {
    println!("the ticket's current estimate is {}", current_estimate)
  }

  let estimate_input = input_getter::get_single_char_input()?;
  let estimate = estimate_input.to_digit(10).ok_or(io::Error::new(
    io::ErrorKind::InvalidInput,
    "couldn't parse estimate from input!",
  ))?;

  Ok(Ticket {
    estimate: Some(estimate),
    ..ticket.clone()
  })
}

fn change_description(ticket: &Ticket) -> Result<Ticket, io::Error> {
  let temp_file_path = &path_provider::get_temp_file_path(&ticket.get_id_as_string());
  {
    let mut file = File::create(temp_file_path)?;
    file.write_all(ticket.description.as_bytes())?;
  }
  // let's just assume everyone's on Linux
  let status = Command::new("vi").arg(&temp_file_path).status()?;
  // todo do something with status
  let mut buf = String::new();
  {
    let mut file = File::open(temp_file_path)?;
    let _ = file.read_to_string(&mut buf)?;
  }
  remove_file(temp_file_path)?;
  Ok(Ticket{description:buf, ..ticket.clone()})
}

fn change_schedule_state(ticket: &Ticket) -> Result<Ticket, io::Error> {
  println!(
    "your ticket's schedule state is currently {}",
    ticket.schedule_state
  );
  println!("what schedule state do you want?");
  let schedule_states = schedule_state_provider::get_schedule_states();

  todo!()
}

fn get_character_schedule_state_mapping(schedule_states: &[String]) -> HashMap<char, String> {
  todo!()
}
