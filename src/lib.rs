#![forbid(unsafe_code)]
#![allow(clippy::result_large_err)]

/*!
Send notifications via Slack, Telegram, E-Mail, ...

The name of this Rust crate is inspired by the notification arrival sound.

# Usage

Add something like this to your `Cargo.toml` based on what you (or your users) need:
```toml
[dependencies.pling]
version = "…"
features = ["email"]
```

```rust no_run
let notifiers = pling::Notifier::from_env();
for notifier in notifiers {
  notifier.send_sync("Hello world!");
}
```

*/

#[cfg(feature = "serde")]
mod serde_helper;

mod command;
pub use command::Command;

#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "desktop")]
pub use desktop::Desktop;

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
pub use telegram::{TargetChat as TelegramTargetChat, Telegram};

#[cfg(any(feature = "http-sync", feature = "http-async"))]
mod webhook;
#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub use webhook::Webhook;

#[cfg(any(feature = "http-sync", feature = "http-async"))]
pub(crate) const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Notifiers which can be used to provide easily configurable notifications for your application.
///
/// # Examples
/// Loading configuration from environment variables is relatively easy.
/// ```rust no_run
/// let notifiers = pling::Notifier::from_env();
/// for notifier in notifiers {
///   notifier.send_sync("Hello from env!");
/// }
/// ```
///
/// With the `serde-derive` feature you can also load a config via Serde like YAML, TOML or JSON.
/// ```rust ignore
/// let yaml = r#"---
/// - Telegram:
///     bot_token: 123:ABC
///     target_chat: 1234
/// "#;
/// let notifiers: Vec<pling::Notifier> = serde_yaml::from_str(yaml)?;
/// for notifier in notifiers {
///   notifier.send_sync("Hello from yaml!");
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
pub enum Notifier {
    Command(Command),

    #[cfg(feature = "desktop")]
    Desktop(Desktop),

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

impl Notifier {
    #[must_use]
    pub fn from_env() -> Vec<Self> {
        let mut result = Vec::new();

        if let Some(n) = Command::from_env() {
            result.push(n.into());
        }
        #[cfg(feature = "desktop")]
        if let Some(n) = Desktop::from_env() {
            result.push(n.into());
        }
        #[cfg(feature = "email")]
        if let Some(n) = Email::from_env() {
            result.push(n.into());
        }
        #[cfg(any(feature = "http-sync", feature = "http-async"))]
        if let Some(n) = Matrix::from_env() {
            result.push(n.into());
        }
        #[cfg(any(feature = "http-sync", feature = "http-async"))]
        if let Some(n) = Slack::from_env() {
            result.push(n.into());
        }
        #[cfg(any(feature = "http-sync", feature = "http-async"))]
        if let Some(n) = Telegram::from_env() {
            result.push(n.into());
        }
        #[cfg(any(feature = "http-sync", feature = "http-async"))]
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

            #[cfg(feature = "desktop")]
            Self::Desktop(o) => o.send_sync(text)?,

            #[cfg(feature = "email")]
            Self::Email(o) => o.send_sync(text)?,

            #[cfg(feature = "http-sync")]
            Self::Matrix(o) => o.send_sync(text)?,
            #[cfg(feature = "http-sync")]
            Self::Slack(o) => o.send_sync(text)?,
            #[cfg(feature = "http-sync")]
            Self::Telegram(o) => o.send_sync(text)?,
            #[cfg(feature = "http-sync")]
            Self::Webhook(o) => o.send_sync(text)?,

            #[cfg(all(feature = "http-async", not(feature = "http-sync")))]
            Self::Matrix(_) | Self::Slack(_) | Self::Telegram(_) | Self::Webhook(_) => {
                unimplemented!("http-sync feature is disabled")
            }
        }
        Ok(())
    }

    #[allow(clippy::unused_async)]
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

            #[cfg(feature = "desktop")]
            Self::Desktop(o) => o.send_sync(text)?,

            #[cfg(feature = "email")]
            Self::Email(o) => o.send_sync(text)?,

            #[cfg(feature = "http-async")]
            Self::Matrix(o) => o.send_async(text).await?,
            #[cfg(feature = "http-async")]
            Self::Slack(o) => o.send_async(text).await?,
            #[cfg(feature = "http-async")]
            Self::Telegram(o) => o.send_async(text).await?,
            #[cfg(feature = "http-async")]
            Self::Webhook(o) => o.send_async(text).await?,

            #[cfg(all(feature = "http-sync", not(feature = "http-async")))]
            Self::Matrix(_) | Self::Slack(_) | Self::Telegram(_) | Self::Webhook(_) => {
                unimplemented!("http-async feature is disabled")
            }
        }
        Ok(())
    }
}
