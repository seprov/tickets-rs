use std::error::Error;

use crate::{
  data_accessors::ticket_da::TicketDa, models::ticket::Ticket,
  user_input_acceptors::stdin_ticket_id_getter,
};

pub struct TicketReadingWorker<'a, T: TicketDa> {
  ticket_da: &'a T,
}

impl<'a, T: TicketDa> TicketReadingWorker<'a, T> {
  // create struct and trait impl
  pub fn read_ticket(&self) -> Result<Ticket, Box<dyn Error>> {
    println!("what's your ticket id?");
    let input = stdin_ticket_id_getter::get_ticket_id()?;
    Ok(self.ticket_da.load_ticket(&input)?)
  }

  pub fn new(ticket_da: &'a T) -> Self {
    Self { ticket_da }
  }
}
