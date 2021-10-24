use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/matrix.md")]
pub struct Matrix {
    pub homeserver: Url,
    pub room_id: String,
    pub access_token: String,
}

impl From<Matrix> for crate::Notification {
    fn from(matrix: Matrix) -> Self {
        Self::Matrix(matrix)
    }
}

impl Matrix {
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let homeserver = std::env::var("MATRIX_HOMESERVER").ok()?.parse().ok()?;
        let room_id = std::env::var("MATRIX_ROOM_ID").ok()?;
        let access_token = std::env::var("MATRIX_ACCESS_TOKEN").ok()?;
        Some(Self {
            homeserver,
            room_id,
            access_token,
        })
    }

    fn generate_url(&self) -> Result<Url, url::ParseError> {
        let path = format!(
            "/_matrix/client/r0/rooms/{}/send/m.room.message?access_token={}",
            self.room_id, self.access_token,
        );
        self.homeserver.join(&path)
    }

    #[cfg(feature = "http-sync")]
    /// Send a Slack notification synchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Slack API.
    pub fn send_sync(&self, text: &str) -> anyhow::Result<()> {
        ureq::post(self.generate_url()?.as_str())
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
    pub async fn send_async(&self, text: &str) -> anyhow::Result<()> {
        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.generate_url()?)
            .body(payload_to_json(text))
            .send()
            .await?
            .error_for_status()?;
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