use serde::{Deserialize, Serialize};

use crate::schedule_state::ScheduleState;

#[derive(Serialize,Deserialize)]
pub struct Ticket {
  pub id: [u8;8],
  pub schedule_state: ScheduleState,
}
