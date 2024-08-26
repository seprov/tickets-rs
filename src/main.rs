use std::io::Error;

use activities::Activities;

use crate::app_state::AppState;

pub mod activities;
pub mod app_state;
pub mod input_getter;
pub mod intro_worker;
pub mod schedule_state;
pub mod ticket;
pub mod ticket_creating_worker;
pub mod ticket_reading_worker;
pub mod ticket_serializer;

pub fn main() {
  let mut app_state = AppState::new();
  let mut current_input = Option::<String>::None;
  let mut current_error = Option::<Error>::None;
  loop {
    match app_state {
      AppState::Greeting => {
        let response = intro_worker::get_intro_choice();
        match response {
          Ok(a) => match a {
            Activities::NewTicket => app_state = AppState::CreatingTicket,
            Activities::ReadTicket => app_state = AppState::ReadingTicket,
          },
          Err(e) => {
            current_error = Some(e);
            app_state = AppState::WrappingUp;
          }
        }
      }
      AppState::CreatingTicket => match ticket_creating_worker::create_ticket() {
        Ok(_) => todo!(),
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
      AppState::ReadingTicket => match ticket_reading_worker::read_ticket() {
        Ok(_) => todo!(),
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
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
