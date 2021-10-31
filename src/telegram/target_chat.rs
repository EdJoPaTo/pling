use std::str::FromStr;

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

impl FromStr for TargetChat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = s.parse::<i64>() {
            Ok(Self::Id(id))
        } else {
            Ok(Self::Username(s.to_string()))
        }
    }
}

impl ToString for TargetChat {
    fn to_string(&self) -> String {
        match self {
            TargetChat::Id(id) => id.to_string(),
            TargetChat::Username(username) => username.to_string(),
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
