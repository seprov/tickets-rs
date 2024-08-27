use std::io::ErrorKind;

use crate::{
  activities::Activities,
  input_getter::{get_input, get_single_char_input},
};

pub fn get_intro_choice() -> Result<Activities, std::io::Error> {
  println!("this is tickets-rs!");
  print!("what would you like to do?");
  print!(
    "
  n: new ticket
  r: read ticket
  "
  );
  
  let c = get_single_char_input()?;
  match c {
    'n' => Ok(Activities::NewTicket),
    'r' => Ok(Activities::ReadTicket),
    _ => todo!(),
  }
}
