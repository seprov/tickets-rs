use std::error::Error;

use crate::{
  data_accessors::ticket_list_provider::TicketListProvider,
  view_providers::ticket_list_view_provider::TicketListViewProvider,
};

pub struct TicketListingWorker<'a, 'b, T: TicketListProvider, U: TicketListViewProvider> {
  ticket_list_provider: &'a T,
  ticket_list_view_provider: &'b U,
}

impl<'a, 'b, T: TicketListProvider, U: TicketListViewProvider> TicketListingWorker<'a, 'b, T, U> {
  pub fn new(ticket_list_provider: &'a T, ticket_list_view_provider: &'b U) -> Self {
    TicketListingWorker {
      ticket_list_provider,
      ticket_list_view_provider,
    }
  }

  pub fn list_tickets(&self) -> Result<(), Box<dyn Error>> {
    let tickets = self.ticket_list_provider.get_ticket_list()?;
    self.ticket_list_view_provider.display_tickets(&tickets);
    Ok(())
  }
}
