//! Gantt task status.

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TaskStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    OnHold,
}

impl TaskStatus {
    /// Input identifier used in `page:gantt` YAML and Gantt JSON.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::OnHold => "on_hold",
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [TaskStatus; 4] = [
        TaskStatus::NotStarted,
        TaskStatus::InProgress,
        TaskStatus::Completed,
        TaskStatus::OnHold,
    ];

    #[test]
    fn defaults_to_not_started() {
        assert_eq!(TaskStatus::default(), TaskStatus::NotStarted);
    }

    #[test]
    fn serializes_as_input_identifier() {
        let json: Vec<String> = ALL
            .iter()
            .map(|status| serde_json::to_string(status).unwrap())
            .collect();
        assert_eq!(
            json,
            [
                r#""not_started""#,
                r#""in_progress""#,
                r#""completed""#,
                r#""on_hold""#
            ]
        );
    }
}
