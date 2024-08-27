pub fn get_ticket_path(ticket_id: &str) -> String {
  format!("data/tickets/{}.json", ticket_id.trim())
}
