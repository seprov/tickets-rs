use crate::{app_state::AppState, input_getter::get_single_char_input};

pub fn get_intro_choice() -> Result<AppState, std::io::Error> {
  println!("\nthis is tickets-rs!");
  print!("what would you like to do?");
  print!(
    "
  t: new ticket
  u: edit ticket
  l: list tickets
  q: quit
"
  );

  let c = get_single_char_input()?;
  match c {
    't' => Ok(AppState::CreatingTicket),
    'u' => Ok(AppState::ReadingTicket),
    'q' => Ok(AppState::WrappingUp),
    'l' => todo!(),
    _ => todo!(),
  }
}

// todo, have this create necessary dirs
