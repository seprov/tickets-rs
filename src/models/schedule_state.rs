use std::{fmt::{self, Display}, ops::Deref, rc::Rc};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ScheduleState {
  value: Rc<String>,
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

impl<'de> Deserialize<'de> for ScheduleState {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
      // Define a visitor to handle the deserialization of the string
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
              })
          }

          fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
          where
              E: de::Error,
          {
              Ok(ScheduleState {
                  value: Rc::new(v),
              })
          }
      }

      // Use the visitor to deserialize the input
      deserializer.deserialize_str(ScheduleStateVisitor)
  }
}

impl ScheduleState {
  pub fn from_str(s: &str) -> Self {
    ScheduleState {
      value: Rc::new(s.to_owned()),
    }
  }
}

impl Deref for ScheduleState {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}
