use url::Url;

/// Matrix Notification
///
/// Documentation: <https://matrix.org/docs/guides/client-server-api/#sending-messages>
#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
    pub homeserver: Url,
    pub room_id: String,
    pub access_token: String,
}

impl Matrix {
    fn generate_url(&self) -> Result<Url, url::ParseError> {
        let Self {
            homeserver,
            room_id,
            access_token,
        } = &self;
        let path = format!(
            "/_matrix/client/r0/rooms/{room_id}/send/m.room.message?access_token={access_token}"
        );
        homeserver.join(&path)
    }

    /// Send a Matrix notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or not be handled by the Matrix API.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> anyhow::Result<()> {
        ureq::post(self.generate_url()?.as_str())
            .header(ureq::http::header::USER_AGENT, crate::USER_AGENT_UREQ)
            .send(payload_to_json(text))?;
        Ok(())
    }

    /// Send a Matrix notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or not be handled by the Matrix API.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> anyhow::Result<()> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT_REQWEST)
            .build()?
            .post(self.generate_url()?)
            .body(payload_to_json(text))
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(reqwest::Error::without_url)?;
        Ok(())
    }
}

#[must_use]
fn payload_to_json(text: &str) -> String {
    format!(
        r#"{{"msgtype":"m.text","body":"{}"}}"#,
        text.replace('"', "\\\"")
    )
}

#[test]
fn generating_payload_works() {
    let result = payload_to_json("hello world");
    assert_eq!(result, r#"{"msgtype":"m.text","body":"hello world"}"#);
}

#[test]
fn generating_payload_with_quotes_works() {
    let result = payload_to_json(r#"hello "world""#);
    assert_eq!(result, r#"{"msgtype":"m.text","body":"hello \"world\""}"#);
}
