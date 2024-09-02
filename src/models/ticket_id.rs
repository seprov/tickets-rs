use std::ops::Deref;

pub struct TicketId {
  value: [u8; 8],
}

impl TicketId {
  pub fn from_bytes(bytes: [u8; 8]) -> Self {
    TicketId { value: bytes }
  }
  pub fn as_string(&self) -> &str {
    todo!()
  }
}

impl Deref for TicketId {
  type Target = [u8;8];

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
