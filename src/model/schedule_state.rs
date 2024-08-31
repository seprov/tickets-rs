use std::ops::Deref;

struct ScheduleState {
  value: String,
}

impl ScheduleState {
  pub fn from_str(s: &str) -> Self {
    ScheduleState {
      value: s.to_owned(),
    }
  }
}

impl Deref for ScheduleState {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
