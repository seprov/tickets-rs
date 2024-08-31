use crate::{adapters::bytes_to_string_converter, model::ticket::Ticket};

pub(crate) fn display_subtickets_short(ticket: &Ticket) -> () {
  println!();
  print!("subtickets currently include: [ ");
  for subticket in &ticket.subtickets {
    print!(
      "{}, ",
      bytes_to_string_converter::get_string_from_bytes(subticket)
    )
  }
  print!("]\n");
}