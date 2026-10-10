//! Inline content of Markdown blocks.

use super::url::{DataImageUri, SafeUrl};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Inline {
    Text(String),
    Code(String),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikethrough(Vec<Inline>),
    Link {
        href: SafeUrl,
        title: Option<String>,
        content: Vec<Inline>,
    },
    Image {
        src: DataImageUri,
        alt: String,
        title: Option<String>,
    },
    SoftBreak,
    HardBreak,
}

/// Flattens inline content to text, as used by `<title>` and heading ids.
///
/// Formatting is dropped, images contribute their alt text, and line breaks become spaces.
pub(crate) fn plain_text(content: &[Inline]) -> String {
    let mut text = String::new();
    push_plain_text(content, &mut text);
    text
}

fn push_plain_text(content: &[Inline], text: &mut String) {
    for inline in content {
        match inline {
            Inline::Text(value) | Inline::Code(value) => text.push_str(value),
            Inline::Emphasis(children)
            | Inline::Strong(children)
            | Inline::Strikethrough(children)
            | Inline::Link {
                content: children, ..
            } => push_plain_text(children, text),
            Inline::Image { alt, .. } => text.push_str(alt),
            Inline::SoftBreak | Inline::HardBreak => text.push(' '),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> Inline {
        Inline::Text(value.to_owned())
    }

    #[test]
    fn flattens_formatting_links_code_and_breaks() {
        let content = vec![
            Inline::Strong(vec![text("출시")]),
            text(" "),
            Inline::Emphasis(vec![Inline::Strikethrough(vec![text("계획")])]),
            Inline::SoftBreak,
            Inline::Link {
                href: SafeUrl::parse("#일정").unwrap(),
                title: Some("일정".to_owned()),
                content: vec![Inline::Code("v1".to_owned())],
            },
            Inline::HardBreak,
            Inline::Image {
                src: DataImageUri::parse("data:image/png;base64,AAAA").unwrap(),
                alt: "로고".to_owned(),
                title: None,
            },
        ];
        assert_eq!(plain_text(&content), "출시 계획 v1 로고");
    }
}
