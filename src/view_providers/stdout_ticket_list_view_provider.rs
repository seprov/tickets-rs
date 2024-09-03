use std::collections::BTreeMap;

use crate::{
  data_accessors::const_str_schedule_state_provider,
  models::{schedule_state::ScheduleState, ticket::Ticket, ticket_id::TicketId},
};

use super::ticket_list_view_provider::TicketListViewProvider;

pub struct StdoutTicketListViewProvider;

impl TicketListViewProvider for StdoutTicketListViewProvider {
  fn display_tickets(&self, tickets: &[crate::models::ticket::Ticket]) {
    println!("\nyour tickets are:");

    let mut schedule_states = self.get_schedule_states_as_btree();
    tickets
      .iter()
      .for_each(|x| self.add_ticket_to_btree(x, &mut schedule_states));

    schedule_states.iter().for_each(|(x, y)| {
      print!("\n{}: ", x);
      y.iter().for_each(|z| print!("{}, ", z))
    });
    println!()
  }
}

impl StdoutTicketListViewProvider {
  fn get_schedule_states_as_btree(&self) -> BTreeMap<ScheduleState, Vec<TicketId>> {
    BTreeMap::<ScheduleState, Vec<TicketId>>::from_iter(
      const_str_schedule_state_provider::get_schedule_states()
        .iter()
        .map(|x| (x.clone(), Vec::new())),
    )
  }

  fn add_ticket_to_btree(&self, ticket: &Ticket, map: &mut BTreeMap<ScheduleState, Vec<TicketId>>) {
    map
      .entry(ticket.schedule_state.clone())
      .or_insert(Vec::new())
      .push(ticket.id.clone());
  }
}
