//! Gantt block model, parsing, validation, and generation boundary.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gantt {
    pub title: String,
    pub tasks: Vec<GanttTask>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GanttTask {
    pub id: String,
    pub name: String,
    pub start: String,
    pub end: String,
    pub progress: u8,
    pub color: String,
}
