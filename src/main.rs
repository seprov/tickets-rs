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
  loop {
    match app_state {
      AppState::Greeting => {
        Greeter::greet();
        match InputGetter::get_input() {
          Ok(input) => current_input = Some(input),
          Err(e) => current_error = Some(e),
        }
        app_state = AppState::WrappingUp;
      }
      AppState::WrappingUp => {
        if let Some(ref x) = current_input {
          println!("Input was: {}", *x);
        }
        if let Some(ref x) = current_error {
          println!("Error was: {}", *x);
        }
        break;
      }
    }
  }
}
