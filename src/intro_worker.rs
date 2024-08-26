use std::io::ErrorKind;

use crate::{activities::Activities, input_getter::get_input};

pub fn get_intro_choice() -> Result<Activities, std::io::Error> {
  println!("this is tickets-rs!");
  print!("what would you like to do?");
  print!(
    "
  n: new ticket
  r: read ticket
  "
  );
  let input = get_input()?;
  if input.len() != 2 {
    Err(std::io::Error::new(
      ErrorKind::InvalidInput,
      format!("you should only enter 1 character!\nyou entered: {}", input),
    ))
  } else {
    match input.chars().nth(0) {
      Some(c) => match c {
        'n' => Ok(Activities::NewTicket),
        'r' => Ok(Activities::ReadTicket),
        _ => todo!(),
      },
      None => todo!(),
    }
  }
}
