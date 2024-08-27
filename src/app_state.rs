#[derive(PartialEq)]
pub enum AppState {
  Greeting,
  CreatingTicket,
  ReadingTicket,
  WrappingUp,
  HandlingTicket,
}

impl AppState {
  pub(crate) fn new() -> Self {
    AppState::Greeting
  }
}
