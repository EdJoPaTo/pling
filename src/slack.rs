use url::Url;

/// Documentation: <https://api.slack.com/messaging/webhooks#getting_started>
///
/// TLDR:
/// - Create app
/// - Enable Incoming Webhooks
/// - Use the <https://hooks.slack.com/…> URL
pub struct Slack {
    pub webhook: Url,
}

impl Slack {
    /// Send a Slack notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Slack API.
    #[allow(clippy::result_large_err)]
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> Result<(), ureq::Error> {
        ureq::post(self.webhook.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(&payload_to_json(text))?;
        Ok(())
    }

    /// Send a Slack notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Slack API.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> reqwest::Result<()> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.webhook.clone())
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
