use std::{collections::HashMap, error::Error};

use crate::{
  data_accessors::const_str_schedule_state_provider,
  models::{schedule_state::ScheduleState, ticket::Ticket, ticket_id::TicketId},
};

pub fn read_subtickets(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
  let mut hashmap = get_schedule_states_as_hashmap();
  add_ticket_to_hashmap(ticket, &mut hashmap);
  for st in &ticket.subtickets {}

  todo!();
}

fn get_schedule_states_as_hashmap() -> HashMap<ScheduleState, Vec<TicketId>> {
  HashMap::<ScheduleState, Vec<TicketId>>::from_iter(
    const_str_schedule_state_provider::get_schedule_states()
      .iter()
      .map(|x| (x.clone(), Vec::new())),
  )
}

fn add_ticket_to_hashmap(ticket: &Ticket, hashmap: &mut HashMap<ScheduleState, Vec<TicketId>>) {
  hashmap
    .entry(ticket.schedule_state.clone())
    .or_insert(Vec::new())
    .push(ticket.id.clone());
}
