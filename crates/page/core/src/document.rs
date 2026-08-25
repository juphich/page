//! PAGE intermediate document model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDocument {
    pub title: String,
    pub blocks: Vec<PageBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageBlock {
    Gantt(crate::gantt::Gantt),
}
