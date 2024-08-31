pub(crate) fn get_string_from_bytes(bytes: &[u8; 8]) -> String {
  bytes
    .iter()
    .filter(|&&b| b != 0)
    .map(|&b| b as char)
    .collect::<String>()
}
