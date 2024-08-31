use crate::models::schedule_state::ScheduleState;

pub const SCHEDULE_STATES: [&'static str; 4] = ["Idea", "Defined", "In-progress", "Released"];

pub fn get_schedule_states() -> Vec<ScheduleState> {
  SCHEDULE_STATES.map(|x| ScheduleState::from_str(x)).to_vec()
}
