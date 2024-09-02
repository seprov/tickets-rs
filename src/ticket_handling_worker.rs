use std::{
  collections::BTreeMap,
  error::Error,
  fs::{remove_file, File},
  io::{self, Read, Write},
  process::Command,
};

use crate::{
  adapters::bytes_to_string_converter,
  data_accessors::{const_str_schedule_state_provider, path_provider},
  models::{app_state::AppState, schedule_state::ScheduleState, ticket::Ticket},
  user_input_acceptors::{
    stdin_input_getter,
    stdin_ticket_id_getter::StdinTicketIdGetter, ticket_id_getter::TicketIdGetter,
  },
  view_providers::{stdout_subticket_view_provider, subticket_view_provider},
};

pub struct TicketHandlingWorker {
  stdin_ticket_id_getter: StdinTicketIdGetter,
}

impl TicketHandlingWorker {
  pub fn handle_ticket(&self, ticket: &Ticket) -> Result<(Ticket, AppState), Box<dyn Error>> {
    // stdout specific view for prompting user for next action
    println!("\nokay, lets work on ticket {}", ticket.get_id_as_string());
    print!("what would you like to do?");
    print!(
      "
  s: change schedule state
  d: add description
  p: point
  r: read ticket details
  t: add subticket id
  u: read subticket details
  v: remove subticket id
  x: save and close the ticket
"
    );

    // stdin specific action picking
    let c = stdin_input_getter::get_single_char_input()?;

    // this is kind of a hack but it requires fewer code changes right now
    if c == 'x' {
      return Ok((ticket.clone(), AppState::Greeting));
    }

    let r: Result<Ticket, Box<dyn Error>> = match c {
      's' => Self::change_schedule_state(ticket),
      'd' => Self::change_description(ticket),
      'p' => Self::change_estimate(ticket),
      'r' => Self::read_ticket(ticket),
      't' => Self::add_subticket_id(self, ticket),
      'u' => subticket_view_provider::read_subtickets(ticket),
      'v' => Self::remove_subticket_id(ticket),
      _ => Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("you entered {}, which is not valid!", c),
      ))),
    };
    r.map(|t| (t, AppState::HandlingTicket))
  }

  fn add_subticket_id(&self, ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    stdout_subticket_view_provider::display_subtickets_short(ticket);
    println!("Please enter the ticket id you'd like to add as a subticket:");
    let ticket_id = self.stdin_ticket_id_getter.get_ticket_id()?;
    let mut t = ticket.clone();
    t.subtickets.push(*ticket_id);
    Ok(t)
  }

  fn remove_subticket_id(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    stdout_subticket_view_provider::display_subtickets_short(ticket);
    todo!()
  }

  fn read_ticket(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    println!();
    println!("id............: {}", ticket.get_id_as_string());
    println!("schedule state: {}", ticket.schedule_state);
    println!(
      "subtickets....: {:?}",
      ticket
        .subtickets
        .iter()
        .map(|x| bytes_to_string_converter::get_string_from_bytes(x))
        .collect::<Vec<String>>()
    );
    println!("estimate......: {}", {
      match ticket.estimate {
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      }
    });
    println!(
      "description...:\n  {}",
      ticket.description.trim().replace("\n", "\n  ")
    );
    Ok(ticket.clone())
  }

  fn change_estimate(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    if let Some(current_estimate) = ticket.estimate {
      println!("the ticket's current estimate is {}", current_estimate)
    }
    println!("what would you like to set the estimate to?");

    let estimate_input = stdin_input_getter::get_single_char_input()?;
    let estimate = estimate_input.to_digit(10).ok_or(io::Error::new(
      io::ErrorKind::InvalidInput,
      "couldn't parse estimate from input!",
    ))?;

    Ok(Ticket {
      estimate: Some(estimate),
      ..ticket.clone()
    })
  }

  fn change_description(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    let temp_file_path = &path_provider::get_temp_file_path(&ticket.get_id_as_string());
    {
      let mut file = File::create(temp_file_path)?;
      file.write_all(ticket.description.as_bytes())?;
    }
    // let's just assume everyone's on Linux :)
    let status = Command::new("vi").arg(&temp_file_path).status()?;

    Self::validate_exit_status(status)?;

    let mut buf = String::new();
    {
      let mut file = File::open(temp_file_path)?;
      let _ = file.read_to_string(&mut buf)?;
    }
    remove_file(temp_file_path)?;
    Ok(Ticket {
      description: buf,
      ..ticket.clone()
    })
  }

  fn validate_exit_status(status: std::process::ExitStatus) -> Result<(), Box<dyn Error>> {
    match status.code() {
      Some(0) => Ok(()),
      Some(sc) => Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        format!("status code was: {}", sc),
      ))),
      None => Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "did not get a status code!",
      ))),
    }
  }

  fn change_schedule_state(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
    println!(
      "your ticket's schedule state is currently: {}",
      ticket.schedule_state
    );
    println!("what schedule state do you want?");
    let schedule_states = const_str_schedule_state_provider::get_schedule_states();
    let mapping = Self::get_character_schedule_state_mapping(&schedule_states);
    for (c, s) in &mapping {
      println!("  {}: {}", c, s)
    }
    let input = stdin_input_getter::get_single_char_input()?;
    for c in mapping.keys() {
      if input == *c {
        if let Some(value) = mapping.get(c) {
          return Ok::<Ticket, Box<dyn Error>>(Ticket {
            schedule_state: (value.to_owned()),
            ..ticket.clone()
          });
        } else {
          continue;
        }
      } else {
        continue;
      };
    }
    Err(Box::new(io::Error::new(
      io::ErrorKind::InvalidInput,
      "couldn't match any valid options!",
    )))
  }

  fn get_character_schedule_state_mapping(
    schedule_states: &[ScheduleState],
  ) -> BTreeMap<char, ScheduleState> {
    let mut current_byte = b'a' - 1;
    schedule_states
      .iter()
      .map(|s| {
        current_byte += 1;
        (current_byte as char, s.to_owned())
      })
      .collect()
  }
}
