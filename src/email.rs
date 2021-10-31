use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/email.md")]
pub struct Email {
    pub server: String,
    pub port: Option<u16>,

    pub username: String,
    pub password: String,

    pub from: String,
    pub to: String,
    pub subject: String,
}

impl From<Email> for crate::Notifier {
    fn from(email: Email) -> Self {
        Self::Email(email)
    }
}

impl Email {
    #[must_use]
    /// Load the email config from environment variables.
    ///
    pub fn from_env() -> Option<Self> {
        let server = std::env::var("EMAIL_SERVER").ok()?;
        let port = std::env::var("EMAIL_PORT")
            .ok()
            .and_then(|s| s.parse().ok());

        let username = std::env::var("EMAIL_USERNAME").ok()?;
        let password = std::env::var("EMAIL_PASSWORD").ok()?;

        let from = std::env::var("EMAIL_FROM").ok()?;
        let to = std::env::var("EMAIL_TO").ok()?;
        let subject = std::env::var("EMAIL_SUBJECT").ok()?;

        Some(Self {
            server,
            port,
            username,
            password,
            from,
            to,
            subject,
        })
    }

    /// Send a Mail synchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the mail could not be built or sent.
    pub fn send_sync(&self, text: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(self.to.parse()?)
            .subject(&self.subject)
            .body(text.to_string())?;

        let creds = Credentials::new(self.username.to_string(), self.password.to_string());

        // Open a remote connection to gmail
        let mut mailer = SmtpTransport::relay(&self.server)?.credentials(creds);

        if let Some(port) = self.port {
            mailer = mailer.port(port);
        }

        mailer.build().send(&email)?;
        Ok(())
    }
}
