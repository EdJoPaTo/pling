use url::Url;

pub struct Webhook {
    pub webhook: Url,
}

impl Webhook {
    /// Send a Webhook via [`ureq`] to the given URL.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    #[allow(clippy::result_large_err)]
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, body: &str) -> Result<(), ureq::Error> {
        ureq::post(self.webhook.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(body)?;
        Ok(())
    }

    /// Send a Webhook via [`reqwest`] to the given URL.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or when the target server returns a not successful status.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, body: &str) -> reqwest::Result<()> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.webhook.clone())
            .body(body.to_owned())
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(reqwest::Error::without_url)?;
        Ok(())
    }
}
