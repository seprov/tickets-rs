use std::error::Error;

use crate::{ data_accessors::ticket_da, models::{ticket::Ticket, ticket_id::TicketId}, user_input_acceptors::stdin_ticket_id_getter};


// create struct and trait impl
pub fn read_ticket() -> Result<Ticket, Box<dyn Error>> {
  println!("what's your ticket id?");
  let input = stdin_ticket_id_getter::get_ticket_id()?;
  get_ticket(input)
}

// create struct and trait impl
pub fn get_ticket(input: TicketId) -> Result<Ticket, Box<dyn Error>> {
  Ok(ticket_da::load_ticket(&input)?)
}
