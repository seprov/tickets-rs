use crate::{data_accessors::path_provider, models::ticket::Ticket, ticket_serializer};

pub fn save_ticket(x: &Ticket) {
  let file_path = path_provider::get_ticket_path(&x.id);
  match ticket_serializer::serialize(&file_path, x) {
    Ok(_) => println!("Saved open ticket: {}", x.id.to_string()),
    Err(_) => println!("Failed to save ticket {}!", x.id.to_string()),
  }
}
