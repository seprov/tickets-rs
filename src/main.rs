use std::io::{self, Error};

use activities::Activities;
use ticket::Ticket;

use crate::app_state::AppState;

pub mod activities;
pub mod app_state;
pub mod input_getter;
pub mod intro_worker;
pub mod path_provider;
pub mod schedule_state_provider;
pub mod ticket;
pub mod ticket_creating_worker;
pub mod ticket_handling_worker;
pub mod ticket_reading_worker;
pub mod ticket_serializer;
pub mod bytes_to_string_converter;
pub mod ticket_id_getter;
pub mod ticket_id_validator;

pub fn main() {
  let mut app_state = AppState::new();
  let mut current_ticket = Option::<Ticket>::None;
  let mut current_error = Option::<Error>::None;
  loop {
    match app_state {
      AppState::Greeting => {
        let response = intro_worker::get_intro_choice();
        match response {
          Ok(a) => match a {
            Activities::NewTicket => app_state = AppState::CreatingTicket,
            Activities::EditTicket => app_state = AppState::ReadingTicket,
            Activities::WrapUp => app_state = AppState::WrappingUp
          },
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
            current_ticket = Some(t.0);
            if t.1 == Activities::WrapUp {
              app_state = AppState::WrappingUp
            }
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
          let file_path = path_provider::get_ticket_path(&x.get_id_as_string());
          match ticket_serializer::serialize(&file_path, x) {
            Ok(_) => println!("Saved open ticket: {}", x.get_id_as_string()),
            Err(_) => println!("Failed to save ticket {}!", x.get_id_as_string()),
          }
        }
        break;
      }
    }
  }
}
