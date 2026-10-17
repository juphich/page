//! Gantt chart structure: groups and tasks in input order.

use serde::{Serialize, Serializer};

use crate::gantt::date::DateRange;
use crate::gantt::id::ItemId;
use crate::gantt::status::TaskStatus;

/// One `page:gantt` block. Serialized as its JSON data block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Gantt {
    pub(crate) version: SchemaVersion,
    pub(crate) title: String,
    items: Vec<GanttItem>,
}

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

/// Item directly under a chart.
///
/// Each item struct writes its own `type` key, so the enum adds no tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub(crate) enum GanttItem {
    Task(GanttTask),
    Epic(Epic),
    Milestone(Milestone),
}

/// Item under a milestone. Has no milestone variant, so milestones cannot nest.
///
/// Each item struct writes its own `type` key, so the enum adds no tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub(crate) enum MilestoneItem {
    Task(GanttTask),
    Epic(Epic),
}

/// Schedule group of tasks and epics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename = "milestone")]
pub(crate) struct Milestone {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    items: Vec<MilestoneItem>,
}

/// Group of tasks. Holds tasks only, so epics cannot nest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename = "epic")]
pub(crate) struct Epic {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    #[serde(rename = "items")]
    tasks: Vec<GanttTask>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename = "task")]
pub(crate) struct GanttTask {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    /// Written as `start` and `end` keys of the task.
    #[serde(flatten)]
    pub(crate) schedule: DateRange,
    pub(crate) status: TaskStatus,
}

/// A chart, milestone, or epic was given no child items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EmptyItems;

impl Gantt {
    pub(crate) fn new(
        version: SchemaVersion,
        title: String,
        items: Vec<GanttItem>,
    ) -> Result<Self, EmptyItems> {
        Ok(Self {
            version,
            title,
            items: non_empty(items)?,
        })
    }

    pub(crate) fn items(&self) -> &[GanttItem] {
        &self.items
    }
}

impl Milestone {
    pub(crate) fn new(
        id: ItemId,
        name: String,
        items: Vec<MilestoneItem>,
    ) -> Result<Self, EmptyItems> {
        Ok(Self {
            id,
            name,
            items: non_empty(items)?,
        })
    }

    pub(crate) fn items(&self) -> &[MilestoneItem] {
        &self.items
    }
}

impl Epic {
    pub(crate) fn new(id: ItemId, name: String, tasks: Vec<GanttTask>) -> Result<Self, EmptyItems> {
        Ok(Self {
            id,
            name,
            tasks: non_empty(tasks)?,
        })
    }

    pub(crate) fn tasks(&self) -> &[GanttTask] {
        &self.tasks
    }
}

fn non_empty<T>(items: Vec<T>) -> Result<Vec<T>, EmptyItems> {
    if items.is_empty() {
        Err(EmptyItems)
    } else {
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gantt::date::CalendarDate;

    fn id(value: &str) -> ItemId {
        ItemId::parse(value).unwrap()
    }

    fn task(task_id: &str, start: &str, end: &str, status: TaskStatus) -> GanttTask {
        let schedule = DateRange::new(
            CalendarDate::parse(start).unwrap(),
            CalendarDate::parse(end).unwrap(),
        )
        .unwrap();
        GanttTask {
            id: id(task_id),
            name: task_id.to_owned(),
            schedule,
            status,
        }
    }

    fn sample_task() -> GanttTask {
        task("t1", "2026-09-01", "2026-09-01", TaskStatus::NotStarted)
    }

    #[test]
    fn rejects_groups_without_items() {
        assert_eq!(
            Gantt::new(SchemaVersion::V1, "일정".to_owned(), Vec::new()),
            Err(EmptyItems)
        );
        assert_eq!(
            Milestone::new(id("m1"), "출시".to_owned(), Vec::new()),
            Err(EmptyItems)
        );
        assert_eq!(
            Epic::new(id("e1"), "기획".to_owned(), Vec::new()),
            Err(EmptyItems)
        );
    }

    #[test]
    fn keeps_items_in_input_order() {
        let first = sample_task();
        let second = task("t2", "2026-09-02", "2026-09-03", TaskStatus::OnHold);
        let epic = Epic::new(
            id("e1"),
            "기획".to_owned(),
            vec![first.clone(), second.clone()],
        )
        .unwrap();
        assert_eq!(epic.tasks(), [first.clone(), second]);

        let milestone = Milestone::new(
            id("m1"),
            "출시".to_owned(),
            vec![
                MilestoneItem::Epic(epic.clone()),
                MilestoneItem::Task(first.clone()),
            ],
        )
        .unwrap();
        assert_eq!(
            milestone.items(),
            [
                MilestoneItem::Epic(epic),
                MilestoneItem::Task(first.clone())
            ]
        );

        let gantt = Gantt::new(
            SchemaVersion::V1,
            "일정".to_owned(),
            vec![GanttItem::Task(first.clone())],
        )
        .unwrap();
        assert_eq!(gantt.items(), [GanttItem::Task(first)]);
    }

    #[test]
    fn serializes_chart_as_json_data_block() {
        let epic = Epic::new(
            id("planning"),
            "기획".to_owned(),
            vec![task(
                "requirements",
                "2026-09-01",
                "2026-09-05",
                TaskStatus::Completed,
            )],
        )
        .unwrap();
        let milestone = Milestone::new(
            id("release"),
            "출시".to_owned(),
            vec![
                MilestoneItem::Epic(epic),
                MilestoneItem::Task(task(
                    "launch",
                    "2026-09-10",
                    "2026-09-10",
                    TaskStatus::NotStarted,
                )),
            ],
        )
        .unwrap();
        let gantt = Gantt::new(
            SchemaVersion::V1,
            "출시 일정".to_owned(),
            vec![
                GanttItem::Task(task(
                    "prepare",
                    "2026-08-29",
                    "2026-08-31",
                    TaskStatus::InProgress,
                )),
                GanttItem::Milestone(milestone),
            ],
        )
        .unwrap();

        let expected = concat!(
            r#"{"version":1,"title":"출시 일정","items":["#,
            r#"{"type":"task","id":"prepare","name":"prepare","start":"2026-08-29","end":"2026-08-31","status":"in_progress"},"#,
            r#"{"type":"milestone","id":"release","name":"출시","items":["#,
            r#"{"type":"epic","id":"planning","name":"기획","items":["#,
            r#"{"type":"task","id":"requirements","name":"requirements","start":"2026-09-01","end":"2026-09-05","status":"completed"}]},"#,
            r#"{"type":"task","id":"launch","name":"launch","start":"2026-09-10","end":"2026-09-10","status":"not_started"}]}]}"#,
        );
        let json = serde_json::to_string(&gantt).unwrap();
        assert_eq!(json, expected);
        assert!(!json.contains(r#""progress":"#));
    }

    #[test]
    fn serializes_schema_version_as_integer() {
        assert_eq!(serde_json::to_string(&SchemaVersion::V1).unwrap(), "1");
    }
}
