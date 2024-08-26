use serde::{Deserialize, Serialize};

// I know... since this is an enum, users can't change it.
// But Rust's enums are so great... just going with it for now.
#[derive(Serialize,Deserialize)]
pub enum ScheduleState {
  Idea,
  Defined,
  InProgress,
  Released,
}
