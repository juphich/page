//! Basic Markdown block model.

use crate::document::inline::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MarkdownBlock {
    Heading {
        level: HeadingLevel,
        content: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    List(List),
    /// Children are Markdown blocks only, so PAGE blocks cannot be nested here.
    BlockQuote(Vec<MarkdownBlock>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Table(Table),
    ThematicBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct List {
    /// `None` for a bullet list, `Some(n)` for an ordered list starting at `n`.
    pub(crate) start: Option<u64>,
    pub(crate) items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListItem {
    /// `None` unless the item is a task list item.
    pub(crate) checked: Option<bool>,
    /// Children are Markdown blocks only, so PAGE blocks cannot be nested here.
    pub(crate) blocks: Vec<MarkdownBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Table {
    pub(crate) header: TableRow,
    pub(crate) rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableRow {
    pub(crate) cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableCell {
    /// Column alignment copied to each cell of the column.
    pub(crate) align: Alignment,
    pub(crate) content: Vec<Inline>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Alignment {
    #[default]
    None,
    Left,
    Center,
    Right,
}
