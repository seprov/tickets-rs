use crate::{path_provider, ticket::Ticket, ticket_serializer};

pub fn save_ticket(x: &Ticket) {
  let file_path = path_provider::get_ticket_path(&x.get_id_as_string());
  match ticket_serializer::serialize(&file_path, x) {
    Ok(_) => println!("Saved open ticket: {}", x.get_id_as_string()),
    Err(_) => println!("Failed to save ticket {}!", x.get_id_as_string()),
  }
}
