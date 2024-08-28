use crate::{activities::Activities, input_getter::get_single_char_input};

pub fn get_intro_choice() -> Result<Activities, std::io::Error> {
  println!("this is tickets-rs!");
  print!("what would you like to do?");
  print!(
    "
  t: new ticket
  u: edit ticket
  q: quit
"
  );

  let c = get_single_char_input()?;
  match c {
    't' => Ok(Activities::NewTicket),
    'u' => Ok(Activities::EditTicket),
    'q' => Ok(Activities::WrapUp),
    _ => todo!(),
  }
}

// todo, have this create necessary dirs
