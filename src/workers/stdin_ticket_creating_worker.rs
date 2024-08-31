use std::error::Error;

use crate::{
  models::ticket::Ticket, data_accessors::json_ticket_id_validator, ticket_serializer,
  user_input_acceptors::stdin_ticket_id_getter,
};

use super::ticket_creating_worker::TicketCreatingWorker;

pub struct StdinTicketCreatingWorker {}

impl TicketCreatingWorker for StdinTicketCreatingWorker {
  fn create_ticket() -> Result<Ticket, Box<dyn Error>> {
    // stdout specific view for creating ticket
    println!("please enter a ticket id for your new ticket");
    // stdin specific user input for getting ticket id
    let (id_string, id_array) = stdin_ticket_id_getter::get_ticket_id()?;

    // ticket json datastore specific ticket already exists validation
    json_ticket_id_validator::ticket_already_exists(&id_string)?;

    let ticket = Ticket::new(id_array, "Idea".to_owned());

    // ticket json datastore specific ticket saving
    let file_path = crate::data_accessors::path_provider::get_ticket_path(&id_string);

    // stdout specific view
    println!("creating ticket");
    // ticket json datastore specific ticket saving
    ticket_serializer::serialize(&file_path, &ticket)?;
    // stdout specific view
    println!("created ticket: {}", id_string);

    Ok(ticket)
  }
}
