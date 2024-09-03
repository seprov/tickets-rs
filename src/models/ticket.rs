use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use super::{schedule_state::ScheduleState, ticket_id::TicketId};

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
  #[serde(
    serialize_with = "serialize_bytes_as_str",
    deserialize_with = "deserialize_bytes_as_str"
  )]
  pub id: TicketId,
  pub schedule_state: ScheduleState,
  pub description: String,
  pub estimate: Option<u32>,
  #[serde(
    serialize_with = "serialize_vec_bytes_as_vec_str",
    deserialize_with = "deserialize_vec_str_as_vec_bytes"
  )]
  pub subtickets: Vec<TicketId>,
}

impl Ticket {
  pub fn new(id: TicketId, schedule_state: ScheduleState) -> Self {
    Self {
      id,
      schedule_state: schedule_state,
      description: "".to_owned(),
      estimate: None,
      subtickets: Vec::new(),
    }
  }
}

fn serialize_vec_bytes_as_vec_str<S>(vec: &Vec<TicketId>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let mut seq: <S as Serializer>::SerializeSeq = serializer.serialize_seq(Some(vec.len()))?;
  let string_ids = vec.iter().map(|x| x.to_string());
  for id in string_ids {
    seq.serialize_element(&id)?;
  }
  seq.end()
}

fn deserialize_vec_str_as_vec_bytes<'de, D>(deserializer: D) -> Result<Vec<TicketId>, D::Error>
where
  D: Deserializer<'de>,
{
  let string_vec = Vec::<String>::deserialize(deserializer)?;

  let mut byte_vec = Vec::with_capacity(string_vec.len());
  for s in string_vec {
    let mut array = [0u8; 8];
    let bytes = s.as_bytes();

    let start_index = 8usize.saturating_sub(bytes.len());
    array[start_index..].copy_from_slice(&bytes[..bytes.len().min(8)]);

    byte_vec.push(TicketId::from(array));
  }

  Ok(byte_vec)
}

fn serialize_bytes_as_str<S>(bytes: &TicketId, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&bytes.to_string())
}

fn deserialize_bytes_as_str<'de, D>(deserializer: D) -> Result<TicketId, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let mut array = [0u8; 8];
  let bytes = s.as_bytes();

  if bytes.len() > 8 {
    return Err(serde::de::Error::custom(
      "String is too long to fit in [u8; 8]",
    ));
  }

  let start_index = 8usize.saturating_sub(bytes.len());
  array[start_index..].copy_from_slice(&bytes[..bytes.len().min(8)]);

  Ok(TicketId::from(array))
}
