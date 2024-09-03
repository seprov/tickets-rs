use super::ticket_list_view_provider::TicketListViewProvider;

pub struct StdoutTicketListViewProvider;

impl TicketListViewProvider for StdoutTicketListViewProvider {
  println!("\nyour tickets are:");
  fn display_tickets(&self, tickets: &[crate::models::ticket::Ticket]) {
    tickets
      .iter()
      .for_each(|x| println!("  {}", x.id.to_string()));
  }
}
