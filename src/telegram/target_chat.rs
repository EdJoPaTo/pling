use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
/// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
pub enum TargetChat {
    Username(String),
    Id(i64),
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
            TargetChat::Username(username) => username.to_string(),
            TargetChat::Id(id) => id.to_string(),
        }
    }
}
