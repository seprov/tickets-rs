use std::ops::Deref;

#[derive(Clone)]
pub struct TicketId {
  value: [u8; 8],
}

impl From<[u8; 8]> for TicketId {
  fn from(value: [u8; 8]) -> Self {
    TicketId { value }
  }
}

impl ToString for TicketId {
  fn to_string(&self) -> String {
    self
      .value
      .iter()
      .filter(|&&b| b != 0)
      .map(|&b| b as char)
      .collect::<String>()
  }
}

impl Deref for TicketId {
  type Target = [u8; 8];

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
