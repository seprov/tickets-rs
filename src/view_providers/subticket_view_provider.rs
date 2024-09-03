use std::error::Error;

use crate::{
  data_accessors::{
    json_ticket_list_provider::JsonTicketListProvider, ticket_list_provider::TicketListProvider,
  },
  models::ticket::Ticket,
};

use super::{
  stdout_ticket_list_view_provider::StdoutTicketListViewProvider,
  ticket_list_view_provider::TicketListViewProvider,
};

pub fn read_subtickets(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
  let ticket_view_provider = StdoutTicketListViewProvider;
  let ticket_list_provider = JsonTicketListProvider;
  let tickets = ticket_list_provider
    .get_ticket_list()?
    .iter()
    .filter(|x| ticket.subtickets.contains(&x.id))
    .map(|x| x.clone())
    .collect::<Vec<Ticket>>();
  ticket_view_provider.display_tickets(&tickets);
  Ok(ticket.clone())
}
