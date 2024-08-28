use serde::{Deserialize, Serialize};

use crate::ticket::Ticket;

#[derive(Serialize,Deserialize)]
pub struct Feature {
  feature_id: [u8;8],
  tickets: Vec<Ticket>
}
