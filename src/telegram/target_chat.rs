#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
/// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()
            .map_or_else(|_| Self::Username(s.to_string()), Self::Id))
    }
}

impl ToString for TargetChat {
    fn to_string(&self) -> String {
        match self {
            Self::Id(id) => id.to_string(),
            Self::Username(username) => username.to_string(),
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
