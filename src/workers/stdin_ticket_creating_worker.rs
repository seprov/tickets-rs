use std::error::Error;

use crate::{
  data_accessors::{json_ticket_id_validator, ticket_da}, models::ticket::Ticket,
  user_input_acceptors::stdin_ticket_id_getter,
};

use super::ticket_creating_worker::TicketCreatingWorker;

pub struct StdinTicketCreatingWorker {}

impl TicketCreatingWorker for StdinTicketCreatingWorker {
  fn create_ticket() -> Result<Ticket, Box<dyn Error>> {
    // stdout specific view for creating ticket
    println!("please enter a ticket id for your new ticket");
    // stdin specific user input for getting ticket id
    let ticket_id = stdin_ticket_id_getter::get_ticket_id()?;
    let id_string = &ticket_id.to_string();

    // ticket json datastore specific ticket already exists validation
    json_ticket_id_validator::ticket_already_exists(&ticket_id)?;

    // ticket json datastore specific ticket saving
    let file_path = crate::data_accessors::path_provider::get_ticket_path(&ticket_id);

    let ticket = Ticket::new(ticket_id, "Idea".to_owned());

    // stdout specific view
    println!("creating ticket");
    // ticket json datastore specific ticket saving
    ticket_da::save_ticket(&ticket)?;
    // stdout specific view
    println!("created ticket: {}", id_string);

    Ok(ticket)
  }
}
