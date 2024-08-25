#[derive(PartialEq)]
pub enum AppState {
  Greeting,
  WrappingUp
}
impl AppState {
  pub(crate) fn new() -> Self {
    AppState::Greeting
  }
}
