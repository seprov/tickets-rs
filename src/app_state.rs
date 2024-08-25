#[derive(PartialEq)]
pub enum AppState {
  Greeting,
  GettingInput,
  WrappingUp,
  HasQuit
}
impl AppState {
  pub(crate) fn new() -> Self {
    AppState::Greeting
  }
}
