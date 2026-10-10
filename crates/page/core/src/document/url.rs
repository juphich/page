//! Validated link and image addresses.

const ALLOWED_LINK_SCHEMES: [&str; 3] = ["http", "https", "mailto"];
const DATA_IMAGE_PREFIX: &str = "data:image/";

/// Link target that can be written into an `href` attribute.
///
/// Allows `http`, `https`, `mailto`, in-document fragments, and relative paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SafeUrl(String);

/// Image source embedded as a `data:image/*` URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DataImageUri(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UrlError {
    Empty,
    /// Leading or trailing whitespace, or a control character anywhere.
    /// Browsers strip these before reading the scheme, so they can hide one.
    UnsafeCharacter,
    /// Lowercased scheme that is not allowed for links.
    DisallowedScheme(String),
    NotDataImage,
}

impl SafeUrl {
    pub(crate) fn parse(value: &str) -> Result<Self, UrlError> {
        check_characters(value)?;
        match scheme(value) {
            Some(scheme) if !ALLOWED_LINK_SCHEMES.contains(&scheme.as_str()) => {
                Err(UrlError::DisallowedScheme(scheme))
            }
            _ => Ok(Self(value.to_owned())),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl DataImageUri {
    pub(crate) fn parse(value: &str) -> Result<Self, UrlError> {
        check_characters(value)?;
        let prefix = value.get(..DATA_IMAGE_PREFIX.len());
        if !prefix.is_some_and(|prefix| prefix.eq_ignore_ascii_case(DATA_IMAGE_PREFIX)) {
            return Err(UrlError::NotDataImage);
        }
        Ok(Self(value.to_owned()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

fn check_characters(value: &str) -> Result<(), UrlError> {
    if value.is_empty() {
        return Err(UrlError::Empty);
    }
    if value.trim() != value || value.chars().any(char::is_control) {
        return Err(UrlError::UnsafeCharacter);
    }
    Ok(())
}

/// Lowercased URL scheme, if `value` starts with one.
///
/// Follows the URL standard: a letter followed by letters, digits, `+`, `-`, or `.`,
/// ending at the first `:`. A `:` after `/`, `?`, or `#` belongs to a relative path.
fn scheme(value: &str) -> Option<String> {
    let end = value.find([':', '/', '?', '#'])?;
    if !value[end..].starts_with(':') {
        return None;
    }
    let candidate = &value[..end];
    let mut chars = candidate.chars();
    let valid = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    valid.then(|| candidate.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_allowed_links() {
        for value in [
            "https://example.com/a?b=c#d",
            "http://example.com",
            "HTTPS://EXAMPLE.COM",
            "mailto:team@example.com",
            "#일정",
            "./docs/plan.md",
            "../plan.md",
            "plan.md",
            "docs/a:b.md",
            "?view=print",
        ] {
            assert_eq!(SafeUrl::parse(value).unwrap().as_str(), value, "{value}");
        }
    }

    #[test]
    fn rejects_disallowed_schemes() {
        for (value, scheme) in [
            ("javascript:alert(1)", "javascript"),
            ("JAVASCRIPT:alert(1)", "javascript"),
            ("JaVaScRiPt:alert(1)", "javascript"),
            ("data:text/html,<script>", "data"),
            ("data:image/png;base64,AAAA", "data"),
            ("file:///etc/passwd", "file"),
            ("vbscript:msgbox", "vbscript"),
        ] {
            assert_eq!(
                SafeUrl::parse(value),
                Err(UrlError::DisallowedScheme(scheme.to_owned())),
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_characters_that_hide_a_scheme() {
        for value in [
            " javascript:alert(1)",
            "javascript:alert(1) ",
            "java\tscript:alert(1)",
            "java\nscript:alert(1)",
            "\u{0}javascript:alert(1)",
        ] {
            assert_eq!(
                SafeUrl::parse(value),
                Err(UrlError::UnsafeCharacter),
                "{value:?}"
            );
        }
        assert_eq!(SafeUrl::parse(""), Err(UrlError::Empty));
    }

    #[test]
    fn accepts_only_data_image_uris() {
        for value in [
            "data:image/png;base64,iVBORw0KGgo=",
            "DATA:IMAGE/svg+xml,<svg/>",
        ] {
            assert_eq!(DataImageUri::parse(value).unwrap().as_str(), value);
        }
        for value in [
            "data:text/html,<script>",
            "https://example.com/a.png",
            "a.png",
            "data:",
        ] {
            assert_eq!(
                DataImageUri::parse(value),
                Err(UrlError::NotDataImage),
                "{value}"
            );
        }
        assert_eq!(
            DataImageUri::parse(" data:image/png;base64,AAAA"),
            Err(UrlError::UnsafeCharacter)
        );
    }
}
