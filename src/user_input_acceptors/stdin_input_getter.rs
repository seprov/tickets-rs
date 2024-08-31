use std::io::{self, ErrorKind};

pub fn get_input() -> io::Result<String> {
  let stdin = std::io::stdin();
  let mut buf: String = String::new();
  match stdin.read_line(&mut buf) {
    Ok(_) => Ok(buf),
    Err(e) => Err(e),
  }
}

pub fn get_single_char_input() -> Result<char, io::Error> {
  let input = get_input()?;
  if input.len() != 2 {
    Err(std::io::Error::new(
      ErrorKind::InvalidInput,
      format!("you should only enter 1 character!\nyou entered: {}", input),
    ))
  } else {
    match input.chars().nth(0) {
      Some(c) => Ok(c),
      None => Err(io::Error::new(
        ErrorKind::InvalidInput,
        format!("couldn't get one char from input chars {}", input),
      )),
    }
  }
}
