use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter, Result as FmtResult};

pub type Id = String;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Reference {
    pub id: Id,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct DateTimeUtc {
    datetime: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ObjectValue {
    Bool(bool),
    String(String),
    // TODO : do we handle that ?
    VecString(Vec<String>),
    F32(f32),
    I32(i32),
    Date(DateTimeUtc),
    Reference(Reference),
    VecReference(Vec<Reference>),
    SubObject(Reference),
    VecSubObjects(Vec<Reference>),
    Null,
}

impl DateTimeUtc {
    pub fn new(datetime: DateTime<Utc>) -> DateTimeUtc {
        return DateTimeUtc {
            datetime: datetime.timestamp(),
        };
    }
    pub fn get_date(&self) -> DateTime<Utc> {
        return DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.datetime, 0), Utc);
    }

    pub fn get_date_as_string(&self) -> String {
        return self.get_date().to_rfc2822();
    }
}

impl Display for DateTimeUtc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.datetime)?;
        return Ok(());
    }
}

pub type ObjectValues = LinkedHashMap<String, ObjectValue>;
