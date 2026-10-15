//! Gantt block model: groups, tasks, and their value types.

// The model has no consumer until the parser (pt-0005) and generator (pt-0007) exist.
// Remove this allowance in pt-0007.
#![allow(dead_code)]

mod date;
mod id;
mod status;

// Unused until pt-0007 for the same reason as above.
#[allow(unused_imports)]
pub(crate) use date::{CalendarDate, DateError, DateRange, DateRangeError};
#[allow(unused_imports)]
pub(crate) use id::{ItemId, ItemIdError};
#[allow(unused_imports)]
pub(crate) use status::TaskStatus;

use serde::{Serialize, Serializer};

/// Supported `page:gantt` schema versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SchemaVersion {
    #[default]
    V1,
}

impl Serialize for SchemaVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::V1 => serializer.serialize_u8(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_schema_version_as_integer() {
        assert_eq!(serde_json::to_string(&SchemaVersion::V1).unwrap(), "1");
    }
}
