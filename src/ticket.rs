use core::str;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::schedule_state::ScheduleState;

#[derive(Serialize, Deserialize)]
pub struct Ticket {
  #[serde(
    serialize_with = "serialize_bytes_as_str",
    deserialize_with = "deserialize_bytes_as_str"
  )]
  pub id: [u8; 8],
  pub schedule_state: ScheduleState,
}

fn serialize_bytes_as_str<S>(bytes: &[u8; 8], serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let s: String = bytes
    .iter()
    .filter(|&&b| b != 0)
    .map(|&b| b as char)
    .rev()
    .collect();
  serializer.serialize_str(&s)
}

fn deserialize_bytes_as_str<'de, D>(deserializer: D) -> Result<[u8; 8], D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let mut array = [0u8; 8];
  let bytes = s.as_bytes();

  let start_index = 8usize.saturating_sub(bytes.len());
  array[start_index..].copy_from_slice(&bytes[..bytes.len().min(8)]);

  Ok(array)
}
