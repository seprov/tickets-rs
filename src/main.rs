use std::io::Error;

use greeter::Greeter;
use input_getter::InputGetter;

use crate::app_state::AppState;

pub mod app_state;
pub mod greeter;
pub mod input_getter;
pub mod schedule_state;
pub mod ticket;

pub fn main() {
  let mut app_state = AppState::new();
  let mut current_input = Option::<String>::None;
  let mut current_error = Option::<Error>::None;
  while app_state != AppState::HasQuit {
    match app_state {
      AppState::Greeting => {
        Greeter::greet();
        app_state = AppState::GettingInput;
      }
      AppState::GettingInput => match InputGetter::get_input() {
        Ok(input) => {
          current_input = Some(input);
          app_state = AppState::WrappingUp;
        }
        Err(e) => current_error = Some(e),
      },
      AppState::WrappingUp => {
        match current_input {
          Some(ref x) => println!("Input was: {}", *x),
          None => (),
        }
        match current_error {
          Some(ref x) => println!("Error was: {}", *x),
          None => (),
        }
        app_state = AppState::HasQuit;
      }
      AppState::HasQuit => panic!("How did this happen!"),
    }
  }
}
