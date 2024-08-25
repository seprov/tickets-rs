use std::io;

pub struct InputGetter {}

impl InputGetter {
  pub fn get_input() -> io::Result<String> {
    let stdin = std::io::stdin();
    let mut buf: String = String::new();
    match stdin.read_line(&mut buf) {
      Ok(_) => Ok(buf),
      Err(e) => Err(e),
    }
  }
}
