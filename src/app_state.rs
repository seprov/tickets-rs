#[derive(PartialEq)]
pub enum AppState {
  Greeting,
  CreatingTicket,
  ReadingTicket,
  WrappingUp
}

impl AppState {
  pub(crate) fn new() -> Self {
    AppState::Greeting
  }
}
