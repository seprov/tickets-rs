use chrono;

pub fn get_ticket_path(ticket_id: &str) -> String {
  format!("data/tickets/{}.json", ticket_id.trim())
}

pub fn get_temp_file_path(ticket_id: &str) -> String {
  format!("data/tmp/{}-{}.txt",ticket_id.trim(), chrono::Utc::now())
}
