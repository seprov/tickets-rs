pub fn get_ticket_path(ticket_id: &str) -> String {
  format!("data/tickets/{}.json", ticket_id.trim())
}

pub fn get_ticket_path_from_bytes(bytes: [u8; 8]) -> String {
  let s: String = bytes
    .iter()
    .filter(|&&b| b != 0)
    .map(|&b| b as char)
    .rev()
    .collect();
  get_ticket_path(&s)
}
