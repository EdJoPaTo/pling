/*!
Send notifications via Slack, Telegram, ...

The name of this Rust crate is inspired by the notification arrival sound.

# Usage

```rust no_run
let notifiers = pling::Notifier::from_env();
for notifier in notifiers {
  notifier.send_ureq("Hello world!");
}
```

*/

mod matrix;
mod slack;
mod telegram;
mod webhook;

pub use matrix::Matrix;
pub use slack::Slack;
pub use telegram::{TargetChat as TelegramTargetChat, Telegram};
pub use webhook::Webhook;

pub(crate) const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

/// Notifiers which can be used to provide easily configurable notifications for your application.
///
/// # Examples
/// Loading configuration from environment variables is relatively easy.
/// ```rust no_run
/// let notifiers = pling::Notifier::from_env();
/// for notifier in notifiers {
///   notifier.send_ureq("Hello from env!");
/// }
/// ```
#[derive(Clone)]
#[non_exhaustive]
pub enum Notifier {
    Matrix(Matrix),
    Slack(Slack),
    Telegram(Telegram),
    Webhook(Webhook),
}

impl Notifier {
    #[must_use]
    pub fn from_env() -> Vec<Self> {
        let mut result = Vec::new();

        if let Some(n) = Matrix::from_env() {
            result.push(n.into());
        }
        if let Some(n) = Slack::from_env() {
            result.push(n.into());
        }
        if let Some(n) = Telegram::from_env() {
            result.push(n.into());
        }
        if let Some(n) = Webhook::from_env() {
            result.push(n.into());
        }

        result
    }

    /// Send the notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// Check the documentation of the given notification implementation errors for more details.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> anyhow::Result<()> {
        match self {
            Self::Matrix(o) => o.send_ureq(text)?,
            Self::Slack(o) => o.send_ureq(text)?,
            Self::Telegram(o) => o.send_ureq(text)?,
            Self::Webhook(o) => o.send_ureq(text)?,
        }
        Ok(())
    }

    /// Send the notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// Check the documentation of the given notification implementation errors for more details.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> anyhow::Result<()> {
        match self {
            Self::Matrix(o) => o.send_reqwest(text).await?,
            Self::Slack(o) => o.send_reqwest(text).await?,
            Self::Telegram(o) => o.send_reqwest(text).await?,
            Self::Webhook(o) => o.send_reqwest(text).await?,
        }
        Ok(())
    }
}
