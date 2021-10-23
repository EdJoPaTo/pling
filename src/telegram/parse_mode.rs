use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParseMode {
    Html,
    Markdown,
    MarkdownV2,
}

impl FromStr for ParseMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "html" => Ok(Self::Html),
            "markdown" => Ok(Self::Markdown),
            "markdownv2" => Ok(Self::MarkdownV2),
            _ => Err(()),
        }
    }
}

impl ParseMode {
    #[must_use]
    pub fn to_str(&self) -> &'static str {
        match self {
            ParseMode::Html => "HTML",
            ParseMode::Markdown => "Markdown",
            ParseMode::MarkdownV2 => "MarkdownV2",
        }
    }
}

impl Display for ParseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[test]
fn can_serde_parse_parsemode() {
    let data = ParseMode::Markdown;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_parsemode() {
    let data = ParseMode::Markdown;
    crate::test_helper::can_string_parse(&data);
}
