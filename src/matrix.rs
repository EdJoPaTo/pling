use url::Url;

#[derive(Clone, PartialEq, Eq)]
#[doc = include_str!("../docs/matrix.md")]
pub struct Matrix {
    pub homeserver: Url,
    pub room_id: String,
    pub access_token: String,
}

impl From<Matrix> for crate::Notifier {
    fn from(matrix: Matrix) -> Self {
        Self::Matrix(matrix)
    }
}

impl Matrix {
    /// Loads the Matrix config from environment variables.
    /// The following variables are used:
    /// - `MATRIX_HOMESERVER`
    /// - `MATRIX_ROOM_ID`
    /// - `MATRIX_ACCESS_TOKEN`
    ///
    /// When any variable is unset or not valid None is returned.
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

    /// Send a Matrix notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Matrix API.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> anyhow::Result<()> {
        ureq::post(self.generate_url()?.as_str())
            .set("User-Agent", crate::USER_AGENT)
            .send_string(&payload_to_json(text))?;
        Ok(())
    }

    /// Send a Matrix notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Matrix API.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> anyhow::Result<()> {
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
