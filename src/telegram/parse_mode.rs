#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ParseMode {
    #[allow(clippy::upper_case_acronyms)]
    HTML,
    #[deprecated = "Use MarkdownV2 instead"]
    Markdown,
    MarkdownV2,
}

impl core::str::FromStr for ParseMode {
    type Err = &'static str;

    #[allow(deprecated)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "HTML" => Ok(Self::HTML),
            "Markdown" => Ok(Self::Markdown),
            "MarkdownV2" => Ok(Self::MarkdownV2),
            _ => Err("unknown parse_mode"),
        }
    }
}
impl ParseMode {
    #[allow(deprecated)]
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::HTML => "HTML",
            Self::Markdown => "Markdown",
            Self::MarkdownV2 => "MarkdownV2",
        }
    }
}

impl std::fmt::Display for ParseMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.pad(self.to_str())
    }
}
