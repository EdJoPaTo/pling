use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/webhook.md")]
pub struct Webhook {
    pub hook: Url,
}

impl From<Webhook> for crate::Notification {
    fn from(webhook: Webhook) -> Self {
        Self::Webhook(webhook)
    }
}

impl Webhook {
    #[must_use]
    /// Loads the Webhook notification config from environment variables.
    /// The following variables are used:
    /// - `WEBHOOK_URL`
    ///
    /// When `WEBHOOK_URL` is unset or not a valid URL None is returned.
    pub fn from_env() -> Option<Self> {
        let hook = std::env::var("WEBHOOK_URL").ok()?.parse().ok()?;
        Some(Self { hook })
    }

    #[cfg(feature = "http-sync")]
    /// Send a Webhook synchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    pub fn send_sync(&self, text: &str) -> Result<(), ureq::Error> {
        ureq::post(self.hook.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(text)?;
        Ok(())
    }

    #[cfg(feature = "http-async")]
    /// Send a Webhook asynchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    pub async fn send_async(&self, text: &str) -> Result<(), reqwest::Error> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.hook.clone())
            .body(text.to_string())
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
