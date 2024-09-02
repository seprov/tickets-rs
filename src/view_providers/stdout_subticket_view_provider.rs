use crate::models::ticket::Ticket;

pub(crate) fn display_subtickets_short(ticket: &Ticket) -> () {
  println!();
  print!("subtickets currently include: [ ");
  for subticket_id in &ticket.subtickets {
    print!("{}, ", subticket_id.to_string())
  }
  print!("]\n");
}
