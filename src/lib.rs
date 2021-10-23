#![forbid(unsafe_code)]

#[cfg(feature = "serde")]
mod serde_helper;

mod command;
pub use command::Command;

#[cfg(feature = "email")]
mod email;
#[cfg(feature = "email")]
pub use email::Email;

#[cfg(any(feature = "http-sync", feature = "http-async"))]
mod matrix;
#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub use matrix::Matrix;

#[cfg(any(feature = "http-sync", feature = "http-async"))]
mod slack;
#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub use slack::Slack;

#[cfg(any(feature = "http-sync", feature = "http-async"))]
mod telegram;
#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub use telegram::{ParseMode as TelegramParseMode, TargetChat as TelegramTargetChat, Telegram};

#[cfg(any(feature = "http-sync", feature = "http-async"))]
mod webhook;
#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub use webhook::Webhook;

#[cfg(test)]
mod test_helper;

#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub(crate) const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Notification which can be used to provide easily configurable notifications for your application.
///
/// # Examples
/// Loading configuration from environment variables is relatively easy.
/// ```
/// let notifications = pling::Notification::from_env();
/// dbg!(&notifications);
/// for notifier in notifications {
///   // TODO: notifier.send_sync("Hello world!");
/// }
/// ```
///
/// With the `serde-derive` feature you can also load a config via Serde like YAML, TOML or JSON.
/// ```
/// let yaml = r#"---
/// - Telegram:
///     bot_token: 123:ABC
///     target_chat: 1234
/// "#;
/// let notifications: Vec<pling::Notification> = serde_yaml::from_str(yaml).expect("failed to parse");
/// dbg!(&notifications);
/// for notifier in notifications {
///   // TODO: notifier.send_sync("Hello world!");
/// }
/// ```
pub enum Notification {
    Command(Command),

    #[cfg(feature = "email")]
    Email(Email),

    #[cfg(any(feature = "http-sync", feature = "http-async"))]
    Matrix(Matrix),

    #[cfg(any(feature = "http-sync", feature = "http-async"))]
    Slack(Slack),

    #[cfg(any(feature = "http-sync", feature = "http-async"))]
    Telegram(Telegram),

    #[cfg(any(feature = "http-sync", feature = "http-async"))]
    Webhook(Webhook),
}

impl Notification {
    #[must_use]
    pub fn from_env() -> Vec<Self> {
        let mut result = Vec::new();

        if let Some(n) = Command::from_env() {
            result.push(n.into());
        }
        if let Some(n) = Email::from_env() {
            result.push(n.into());
        }
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

    /// Send the notification synchronously.
    ///
    /// # Errors
    ///
    /// Check the documentation of the given notification implementation errors for more details.
    ///
    /// # Panics
    ///
    /// When this crate is built with only some features not everything is implemented.
    /// For example it won't work to `Telegram::send_sync` when the feature `http-sync` isn't enabled.
    pub fn send_sync(&self, text: &str) -> anyhow::Result<()> {
        match self {
            Self::Command(cmd) => cmd.send_sync(text)?,

            #[cfg(feature = "email")]
            Self::Email(o) => o.send_sync(text)?,

            #[cfg(feature = "http-sync")]
            Self::Matrix(o) => o.send_sync(text)?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Matrix(_) => unimplemented!("http-sync feature is disabled"),

            #[cfg(feature = "http-sync")]
            Self::Slack(o) => o.send_sync(text)?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Slack(_) => unimplemented!("http-sync feature is disabled"),

            #[cfg(feature = "http-sync")]
            Self::Telegram(o) => o.send_sync(text, None, false, false)?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Telegram(_) => unimplemented!("http-sync feature is disabled"),

            #[cfg(feature = "http-sync")]
            Self::Webhook(o) => o.send_sync(text)?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Webhook(_) => unimplemented!("http-sync feature is disabled"),
        }
        Ok(())
    }

    /// Send the notification asynchronously.
    ///
    /// # Errors
    ///
    /// Check the documentation of the given notification implementation errors for more details.
    ///
    /// # Panics
    ///
    /// When this crate is built with only some features not everything is implemented.
    /// For example it won't work to `Telegram::send_async` when the feature `http-async` isn't enabled.
    pub async fn send_async(&self, text: &str) -> anyhow::Result<()> {
        match self {
            Self::Command(cmd) => cmd.send_sync(text)?,

            #[cfg(feature = "email")]
            Self::Email(o) => o.send_sync(text)?,

            #[cfg(feature = "http-sync")]
            Self::Matrix(o) => o.send_async(text).await?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Matrix(_) => unimplemented!("http-async feature is disabled"),

            #[cfg(feature = "http-sync")]
            Self::Slack(o) => o.send_async(text).await?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Slack(_) => unimplemented!("http-async feature is disabled"),

            #[cfg(feature = "http-async")]
            Self::Telegram(o) => o.send_async(text, None, false, false).await?,
            #[cfg(all(feature = "http-sync", not(feature = "http-async")))]
            Self::Telegram(_) => unimplemented!("http-async feature is disabled"),

            #[cfg(feature = "http-sync")]
            Self::Webhook(o) => o.send_async(text).await?,
            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Webhook(_) => unimplemented!("http-async feature is disabled"),
        }
        Ok(())
    }
}
