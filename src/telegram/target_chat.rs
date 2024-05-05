use std::borrow::Cow;

/// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetChat {
    Id(i64),
    Username(String),
}

impl From<i64> for TargetChat {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl core::str::FromStr for TargetChat {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = str.parse::<i64>() {
            return Ok(Self::Id(id));
        }

        if !str.starts_with('@') {
            return Err("Telegram username needs to start with an @ symbol");
        }
        Ok(Self::Username(str.to_owned()))
    }
}

impl TargetChat {
    #[must_use]
    pub fn to_chat_id(&self) -> Cow<str> {
        match self {
            Self::Id(id) => Cow::Owned(id.to_string()),
            Self::Username(username) => Cow::Borrowed(username),
        }
    }
}

#[test]
fn can_parse_id_from_str() {
    let result = "12345".parse::<TargetChat>().unwrap();
    assert_eq!(result, TargetChat::Id(12345));
}

#[test]
fn can_parse_username_from_str() {
    let result = "@HelloWorld".parse::<TargetChat>().unwrap();
    assert_eq!(result, TargetChat::Username("@HelloWorld".into()));
}

#[test]
#[should_panic = "start with an @ symbol"]
fn username_no_at() {
    let result = "HelloWorld".parse::<TargetChat>().unwrap();
    dbg!(result);
}
