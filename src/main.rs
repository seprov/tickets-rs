use std::{
  error::Error,
  io::{self},
};

use data_accessors::{
  json_ticket_da, json_ticket_list_provider::JsonTicketListProvider, ticket_da::TicketDa,
};
use models::{app_state::AppState, ticket::Ticket};
use view_providers::stdout_ticket_list_view_provider::StdoutTicketListViewProvider;
use workers::{
  stdin_intro_worker, stdin_ticket_creating_worker::StdinTicketCreatingWorker,
  ticket_creating_worker::TicketCreatingWorker, ticket_handling_worker,
  ticket_listing_worker::TicketListingWorker, ticket_reading_worker::TicketReadingWorker,
};

pub mod data_accessors;
pub mod models;
pub mod user_input_acceptors;
pub mod view_providers;
pub mod workers;

pub fn main() {
  let mut app_state = AppState::new();
  let mut current_ticket = Option::<Ticket>::None;
  let mut current_error = Option::<Box<dyn Error>>::None;

  let ticket_da = json_ticket_da::JsonTicketDa {};
  let ticket_reading_worker = TicketReadingWorker::new(&ticket_da);
  let ticket_creating_worker = StdinTicketCreatingWorker::new(&ticket_da);

  let ticket_list_view_provider = StdoutTicketListViewProvider;
  let ticket_list_provider = JsonTicketListProvider;
  let ticket_listing_worker =
    TicketListingWorker::new(&ticket_list_provider, &ticket_list_view_provider);

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
      AppState::ListingTickets => match ticket_listing_worker.list_tickets() {
        Ok(_) => app_state = AppState::Greeting,
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
      AppState::CreatingTicket => match ticket_creating_worker.create_ticket() {
        Ok(t) => {
          current_ticket = Some(t);
          app_state = AppState::HandlingTicket;
        }
        Err(e) => {
          current_error = Some(e);
          app_state = AppState::WrappingUp;
        }
      },
      AppState::ReadingTicket => match ticket_reading_worker.read_ticket() {
        Ok(t) => {
          println!("ticket id read as: {}", t.id.to_string());
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
            match ticket_da.save_ticket(&t.0) {
              Ok(_) => (),
              Err(e) => println!("{}", e),
            }
            current_ticket = Some(t.0);
          }
          Err(e) => {
            current_error = Some(e);
            app_state = AppState::WrappingUp;
          }
        },
        None => {
          current_error = Some(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "we didn't have a ticket",
          )))
        }
      },
      AppState::WrappingUp => {
        if let Some(ref x) = current_error {
          println!("Error was: {}", *x);
        }
        if let Some(ref x) = current_ticket {
          match ticket_da.save_ticket(x) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
          }
        }
        break;
      }
    }
  }
}
