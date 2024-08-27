use std::{io, str::FromStr};

pub const SCHEDULE_STATES: [&'static str; 4] = ["Idea", "Defined", "InProgress", "Released"];

pub fn get_schedule_states() -> Vec<Result<String,io::Error>> {
  SCHEDULE_STATES
    .map(|x| String::from_str(x))
    .as_slice()
    .into_iter()
    .map(|x| match x {
      Ok(s) => Ok(s.clone()), // doing a clone... hopefully this is only temporary
      Err(_) => Err(io::Error::new(io::ErrorKind::Other, "this can't happen right now, since we have an infallible")),
    }).collect()
}
