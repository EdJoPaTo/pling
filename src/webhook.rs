use url::Url;

#[derive(Clone, PartialEq, Eq)]
#[doc = include_str!("../docs/webhook.md")]
pub struct Webhook {
    pub hook: Url,
}

impl From<Webhook> for crate::Notifier {
    fn from(webhook: Webhook) -> Self {
        Self::Webhook(webhook)
    }
}

impl Webhook {
    /// Loads the Webhook config from environment variables.
    /// The following variables are used:
    /// - `WEBHOOK_URL`
    ///
    /// When `WEBHOOK_URL` is unset or not a valid URL None is returned.
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let hook = std::env::var("WEBHOOK_URL").ok()?.parse().ok()?;
        Some(Self { hook })
    }

    /// Send a Webhook via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> Result<(), ureq::Error> {
        ureq::post(self.hook.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(text)?;
        Ok(())
    }

    /// Send a Webhook via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> reqwest::Result<()> {
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
