use std::io::{self, Error};

use model::{app_state::AppState, ticket::Ticket};
use user_input::stdin_intro_worker;

pub mod ticket_creating_worker;
pub mod ticket_handling_worker;
pub mod ticket_id_validator;
pub mod ticket_reading_worker;
pub mod ticket_saver;
pub mod ticket_serializer;
pub mod model;
pub mod adapters;
pub mod user_input;
pub mod data_access;
pub mod view;

pub fn main() {
  let mut app_state = AppState::new();
  let mut current_ticket = Option::<Ticket>::None;
  let mut current_error = Option::<Error>::None;
  loop {
    match app_state {
      AppState::Greeting => {
        let response = stdin_intro_worker::prompt_for_activity();
        match response {
          Ok(a) => app_state = a,
          Err(e) => {
            current_error = Some(e);
            app_state = AppState::WrappingUp;
          }
        }
      }
      AppState::CreatingTicket => match ticket_creating_worker::create_ticket() {
        Ok(t) => {
          current_ticket = Some(t);
          app_state = AppState::HandlingTicket;
        }
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
      AppState::ReadingTicket => match ticket_reading_worker::read_ticket() {
        Ok(t) => {
          println!("ticket id read as: {}", t.get_id_as_string());
          current_ticket = Some(t);
          app_state = AppState::HandlingTicket;
        }
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
      AppState::HandlingTicket => match current_ticket {
        Some(ref ticket) => match ticket_handling_worker::handle_ticket(ticket) {
          Ok(t) => {
            app_state = t.1;
            ticket_saver::save_ticket(&t.0);
            current_ticket = Some(t.0);
          }
          Err(e) => {
            current_error = Some(e);
            app_state = AppState::WrappingUp;
          }
        },
        None => {
          current_error = Some(io::Error::new(
            io::ErrorKind::InvalidData,
            "we didn't have a ticket",
          ))
        }
      },
      AppState::WrappingUp => {
        if let Some(ref x) = current_error {
          println!("Error was: {}", *x);
        }
        if let Some(ref x) = current_ticket {
          ticket_saver::save_ticket(x);
        }
        break;
      }
    }
  }
}
