use std::{
    error::Error,
    fmt::Display,
    i64,
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

use chrono::{DateTime, Local, TimeZone};

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) completed: bool,
    pub(crate) deleted: bool,
    pub(crate) created_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
    pub(crate) deleted_at: Option<i64>,
}

impl Item {
    /// Associated Functions,
    /// which first parameter is NOT `&self(self: &Self)`,
    /// `&mut self(self: &mut Self)` or `self(self: Self)`
    pub fn new(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self {
        Item {
            id,
            name: name.to_string(),
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_prettier_string(&self) -> String {
        let created_at = timestamp_to_datetime_string(self.created_at);
        let completed_at = timestamp_to_datetime_string(self.completed_at);
        let deleted_at = timestamp_to_datetime_string(self.deleted_at);

        let mut result = format!(
            "{:3} {} {} {}\n\n",
            self.id,
            if self.completed {
                "\u{2705}"
            } else {
                "\u{1f532}"
            },
            if self.deleted { "\u{1f6ae}" } else { "" },
            self.name,
        );

        if !created_at.is_empty() {
            result += &format!("\tCreated at: {}\n", created_at);
        }
        if !completed_at.is_empty() {
            result += &format!("\tCompleted at: {}\n", completed_at);
        }
        if !deleted_at.is_empty() {
            result += &format!("\tDeleted at: {}\n", deleted_at);
        }

        result
    }
}

/// Serialization
impl ToString for Item {
    fn to_string(&self) -> String {
        let created_at = timestamp_to_raw_string(self.created_at);
        let completed_at = timestamp_to_raw_string(self.completed_at);
        let deleted_at = timestamp_to_raw_string(self.deleted_at);

        let name = self
            .name
            .replace(",", COMMA_FAKE)
            .replace(r"\n", NEWLINE_FAKE);

        format!(
            "{},{},{},{},{},{},{}",
            self.id, name, self.completed, self.deleted, created_at, completed_at, deleted_at,
        )
    }
}

/// Deserialization
impl FromStr for Item {
    type Err = ParseItemError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let splitted = s.split(",").collect::<Vec<_>>();
        if splitted.len() != ITEM_COUNT {
            return Err(ParseItemError(format!(
                "Expected {} properties, found {}",
                ITEM_COUNT,
                splitted.len()
            )));
        }

        let id = splitted[0].parse::<u32>()?;
        let name = &splitted[1]
            .replace(COMMA_FAKE, ",")
            .replace(NEWLINE_FAKE, "\n");

        let completed = splitted[2].parse::<bool>()?;
        let deleted = splitted[3].parse::<bool>()?;

        let created_at = str_to_timestamp(splitted[4])?;
        let completed_at = str_to_timestamp(splitted[5])?;
        let deleted_at = str_to_timestamp(splitted[6])?;

        Ok(Item::new(
            id,
            name,
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        ))
    }
}

#[derive(Debug)]
pub struct ParseItemError(String);

type Result<T> = std::result::Result<T, ParseItemError>;

impl Error for ParseItemError {}

impl Display for ParseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Deserialization todo fail: {}", self.0)
    }
}

impl From<ParseIntError> for ParseItemError {
    fn from(value: ParseIntError) -> Self {
        Self(value.to_string())
    }
}

impl From<ParseBoolError> for ParseItemError {
    fn from(value: ParseBoolError) -> Self {
        Self(value.to_string())
    }
}

const ITEM_COUNT: usize = 7;
const COMMA_FAKE: &str = "<@^_fake_comma_$#>";
const NEWLINE_FAKE: &str = "<@^_fake_newline_$#>";

fn timestamp_to_datetime_string(timestamp: Option<i64>) -> String {
    if let Some(timestamp) = timestamp {
        if let Some(utc) = DateTime::from_timestamp(timestamp, 0) {
            Local.from_utc_datetime(&utc.naive_utc()).to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn timestamp_to_raw_string(timestamp: Option<i64>) -> String {
    if let Some(x) = timestamp {
        x.to_string()
    } else {
        String::new()
    }
}

fn str_to_timestamp(s: &str) -> Result<Option<i64>> {
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse::<i64>()?))
    }
}
