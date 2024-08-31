use crate::{model::app_state::AppState, user_input::stdin_input_getter::get_single_char_input};

pub fn prompt_for_activity() -> Result<AppState, std::io::Error> {
  // this is a stdout specific view for picking the app state
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

  // this is a stdin specific user_input for picking the app state
  let c = get_single_char_input()?;
  match c {
    't' => Ok(AppState::CreatingTicket),
    'u' => Ok(AppState::ReadingTicket),
    'l' => todo!(),
    'q' => Ok(AppState::WrappingUp),
    _ => todo!(),
  }
}

// todo, have this create necessary dirs
