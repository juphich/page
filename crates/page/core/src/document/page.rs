//! Top-level PAGE document and its blocks.

use crate::document::block::MarkdownBlock;
use crate::document::inline::{Inline, plain_text};

const DEFAULT_TITLE: &str = "PAGE Document";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageDocument {
    /// Content of the first level-1 heading, which is removed from `blocks`.
    pub(crate) title: Option<Vec<Inline>>,
    /// Sections in source order.
    pub(crate) blocks: Vec<DocumentBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DocumentBlock {
    Markdown(MarkdownBlock),
}

impl PageDocument {
    /// Plain-text title for `<title>`, falling back to `PAGE Document`.
    pub(crate) fn title_text(&self) -> String {
        let text = self.title.as_deref().map(plain_text).unwrap_or_default();
        let text = text.trim();
        if text.is_empty() {
            DEFAULT_TITLE.to_owned()
        } else {
            text.to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(title: Option<Vec<Inline>>) -> PageDocument {
        PageDocument {
            title,
            blocks: Vec::new(),
        }
    }

    #[test]
    fn uses_flattened_heading_as_title() {
        let title = vec![
            Inline::Strong(vec![Inline::Text("출시".to_owned())]),
            Inline::Text(" 계획".to_owned()),
        ];
        assert_eq!(document(Some(title)).title_text(), "출시 계획");
    }

    #[test]
    fn falls_back_to_default_title() {
        assert_eq!(document(None).title_text(), DEFAULT_TITLE);
        assert_eq!(document(Some(Vec::new())).title_text(), DEFAULT_TITLE);
        assert_eq!(
            document(Some(vec![Inline::Text("  ".to_owned())])).title_text(),
            DEFAULT_TITLE
        );
    }
}
