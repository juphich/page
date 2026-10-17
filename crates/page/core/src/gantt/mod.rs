//! Gantt block model: groups, tasks, and their value types.

// The model has no consumer until the parser (pt-0005) and generator (pt-0007) exist.
// Remove this allowance in pt-0007.
#![allow(dead_code)]

mod chart;
mod date;
mod id;
mod status;

// Unused until pt-0007 for the same reason as above.
#[allow(unused_imports)]
pub(crate) use chart::{
    EmptyItems, Epic, Gantt, GanttItem, GanttTask, Milestone, MilestoneItem, SchemaVersion,
};
#[allow(unused_imports)]
pub(crate) use date::{CalendarDate, DateError, DateRange, DateRangeError};
#[allow(unused_imports)]
pub(crate) use id::{ItemId, ItemIdError};
#[allow(unused_imports)]
pub(crate) use status::TaskStatus;
