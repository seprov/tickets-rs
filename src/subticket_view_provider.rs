use std::{collections::HashMap, io};

use crate::{data_access::const_str_schedule_state_provider, model::ticket::Ticket};

pub fn read_subtickets(ticket: &Ticket) -> Result<Ticket, io::Error> {
  let mut hashmap = get_schedule_states_as_hashmap();
  add_ticket_to_hashmap(ticket, &mut hashmap);
  for st in &ticket.subtickets {}

  todo!();
}

fn get_schedule_states_as_hashmap() -> HashMap<String, Vec<[u8; 8]>> {
  HashMap::<String, Vec<[u8; 8]>>::from_iter(
    const_str_schedule_state_provider::get_schedule_states()
      .iter()
      .map(|x| (x.to_owned(), Vec::new())),
  )
}

fn add_ticket_to_hashmap(ticket: &Ticket, hashmap: &mut HashMap<String, Vec<[u8; 8]>>) {
  hashmap
    .entry(ticket.schedule_state.to_owned())
    .or_insert(Vec::new())
    .push(ticket.id);
}
