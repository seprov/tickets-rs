use crate::models::schedule_state::ScheduleState;

pub const SCHEDULE_STATES: [&'static str; 4] = ["Idea", "Defined", "In-progress", "Released"];

pub fn get_schedule_states() -> Vec<ScheduleState> {
  SCHEDULE_STATES.map(ScheduleState::from_str).to_vec()
}

pub fn get_schedule_state_position(s: &ScheduleState) -> Option<u8> {
  SCHEDULE_STATES.iter().position(|&x| **s == x).map(|a| a as u8)
}
