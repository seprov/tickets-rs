#[derive(PartialEq)]
pub enum AppState {
  Greeting,
  CreatingTicket,
  ReadingTicket,
  WrappingUp,
  HandlingTicket,
  ListingTickets,
}

impl AppState {
  pub(crate) fn new() -> Self {
    AppState::Greeting
  }
}
