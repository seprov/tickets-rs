use std::{collections::HashMap, error::Error};

use crate::{
  data_accessors::const_str_schedule_state_provider,
  models::{schedule_state::ScheduleState, ticket::Ticket, ticket_id::TicketId},
};

pub fn read_subtickets(ticket: &Ticket) -> Result<Ticket, Box<dyn Error>> {
  todo!();
}
