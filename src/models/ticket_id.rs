use std::{fmt::Display, ops::Deref};

#[derive(Clone, PartialEq)]
pub struct TicketId {
  value: [u8; 8],
}

impl Display for TicketId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .value
      .iter()
      .filter(|&&b| b != 0)
      .map(|&b| b as char)
      .collect::<String>()
      .fmt(f)
  }
}

impl From<[u8; 8]> for TicketId {
  fn from(value: [u8; 8]) -> Self {
    TicketId { value }
  }
}

impl Deref for TicketId {
  type Target = [u8; 8];

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
