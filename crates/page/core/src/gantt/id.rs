//! Gantt item identifiers.

use serde::Serialize;

const MAX_LEN: usize = 64;

/// Identifier of a milestone, epic, or task.
///
/// Starts with an ASCII letter, continues with ASCII letters, digits, `_`, or `-`,
/// and is 1 to 64 characters long.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub(crate) struct ItemId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ItemIdError {
    Empty,
    TooLong,
    InvalidStart,
    InvalidCharacter(char),
}

impl ItemId {
    pub(crate) fn parse(value: &str) -> Result<Self, ItemIdError> {
        let mut chars = value.chars();
        let first = chars.next().ok_or(ItemIdError::Empty)?;
        if value.chars().count() > MAX_LEN {
            return Err(ItemIdError::TooLong);
        }
        if !first.is_ascii_alphabetic() {
            return Err(ItemIdError::InvalidStart);
        }
        if let Some(invalid) = chars.find(|&c| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
        {
            return Err(ItemIdError::InvalidCharacter(invalid));
        }
        Ok(Self(value.to_owned()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_letters_digits_underscore_and_hyphen() {
        for value in ["a", "Task_1", "release-2026", &"a".repeat(MAX_LEN)] {
            assert_eq!(ItemId::parse(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn rejects_invalid_identifiers() {
        assert_eq!(ItemId::parse(""), Err(ItemIdError::Empty));
        assert_eq!(
            ItemId::parse(&"a".repeat(MAX_LEN + 1)),
            Err(ItemIdError::TooLong)
        );
        assert_eq!(ItemId::parse("1abc"), Err(ItemIdError::InvalidStart));
        assert_eq!(ItemId::parse("-abc"), Err(ItemIdError::InvalidStart));
        assert_eq!(
            ItemId::parse("a.b"),
            Err(ItemIdError::InvalidCharacter('.'))
        );
        assert_eq!(
            ItemId::parse("a b"),
            Err(ItemIdError::InvalidCharacter(' '))
        );
        assert_eq!(ItemId::parse("작업"), Err(ItemIdError::InvalidStart));
    }

    #[test]
    fn serializes_as_plain_string() {
        let id = ItemId::parse("epic-0001").unwrap();
        assert_eq!(serde_json::to_string(&id).unwrap(), r#""epic-0001""#);
    }
}
