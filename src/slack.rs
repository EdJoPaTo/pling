use url::Url;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/slack.md")]
pub struct Slack {
    pub hook: Url,
}

impl From<Slack> for crate::Notifier {
    fn from(slack: Slack) -> Self {
        Self::Slack(slack)
    }
}

impl Slack {
    #[must_use]
    /// Loads the Slack config from environment variables.
    /// The following variables are used:
    /// - `SLACK_HOOK`
    ///
    /// When `SLACK_HOOK` is unset or not a valid URL None is returned.
    pub fn from_env() -> Option<Self> {
        let hook = std::env::var("SLACK_HOOK").ok()?.parse().ok()?;
        Some(Self { hook })
    }

    #[cfg(feature = "http-sync")]
    /// Send a Slack notification synchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Slack API.
    pub fn send_sync(&self, text: &str) -> Result<(), ureq::Error> {
        ureq::post(self.hook.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(&payload_to_json(text))?;
        Ok(())
    }

    #[cfg(feature = "http-async")]
    /// Send a Slack notification asynchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Slack API.
    pub async fn send_async(&self, text: &str) -> Result<(), reqwest::Error> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.hook.clone())
            .body(payload_to_json(text))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

fn payload_to_json(text: &str) -> String {
    format!(r#"{{"text":"{}"}}"#, text.replace('"', "\\\""))
}

#[test]
fn generating_payload_works() {
    let result = payload_to_json("hello world");
    assert_eq!(result, r#"{"text":"hello world"}"#);
}

#[test]
fn generating_payload_with_quotes_works() {
    let result = payload_to_json(r#"hello "world""#);
    assert_eq!(result, r#"{"text":"hello \"world\""}"#);
}
