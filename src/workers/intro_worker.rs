use std::error::Error;

use crate::models::app_state::AppState;

pub trait IntroWorker {
  fn prompt_for_activity() -> Result<AppState, Box<dyn Error>>;
}
