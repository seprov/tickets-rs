use std::{
  fmt::{self, Display},
  ops::Deref,
  rc::Rc,
};

use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct ScheduleState {
  value: Rc<String>,
  position: Option<i8>,
}

impl Ord for ScheduleState {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.position {
      Some(s) => match other.position {
        Some(o) => s.cmp(&o),
        None => std::cmp::Ordering::Greater,
      },
      None => match other.position {
        Some(_) => std::cmp::Ordering::Less,
        None => std::cmp::Ordering::Equal,
      },
    }
  }
}

impl Display for ScheduleState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    String::fmt(&self.value, f)
  }
}

impl Serialize for ScheduleState {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.value)
  }
}

// Pure ChatGPT for this
impl<'de> Deserialize<'de> for ScheduleState {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ScheduleStateVisitor;

    impl<'de> Visitor<'de> for ScheduleStateVisitor {
      type Value = ScheduleState;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing the schedule state")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(ScheduleState {
          value: Rc::new(v.to_owned()),
          position: None,
        })
      }

      fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(ScheduleState {
          value: Rc::new(v),
          position: None,
        })
      }
    }

    deserializer.deserialize_str(ScheduleStateVisitor)
  }
}

impl ScheduleState {
  pub fn from_str(s: &str) -> Self {
    ScheduleState {
      value: Rc::new(s.to_owned()),
      position: None,
    }
  }
}

impl Deref for ScheduleState {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
