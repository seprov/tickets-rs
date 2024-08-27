pub const SCHEDULE_STATES: [&'static str; 4] = ["Idea", "Defined", "InProgress", "Released"];

pub fn get_schedule_states() -> Vec<String> {
  SCHEDULE_STATES.map(|x| x.to_owned()).to_vec()
}
