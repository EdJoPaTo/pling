//! Notifications for your [`clap`] arguments.
//!
//! ```
//! use clap::Parser;
//!
//! #[derive(Parser)]
//! pub struct Cli {
//!     #[command(flatten)]
//!     pub notifications: pling::clap::Args,
//! }
//!
//! let matches = Cli::parse();
//! matches.notifications.send_ureq("Hello world!")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![allow(clippy::struct_field_names)]

use url::Url;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub matrix: Matrix,

    #[command(flatten)]
    pub slack: Slack,

    #[command(flatten)]
    pub telegram: Telegram,

    #[command(flatten)]
    pub webhook: Webhook,
}
impl Args {
    /// Send the notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// Check the documentation of the given notification implementation errors for more details.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> anyhow::Result<()> {
        use anyhow::Context as _;

        if let Some(notifier) = self.matrix.to_plain() {
            notifier
                .send_ureq(text)
                .context("Failed to send Matrix notification")?;
        }
        if let Some(notifier) = self.slack.to_plain() {
            notifier
                .send_ureq(text)
                .context("Failed to send Slack notification")?;
        }
        if let Some(notifier) = self.telegram.to_plain() {
            notifier
                .send_ureq(text)
                .context("Failed to send Telegram notification")?;
        }
        if let Some(notifier) = &self.webhook.to_plain() {
            notifier
                .send_ureq(text)
                .context("Failed to send webhook notification")?;
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
        use anyhow::Context as _;

        if let Some(notifier) = self.matrix.to_plain() {
            notifier
                .send_reqwest(text)
                .await
                .context("Failed to send Matrix notification")?;
        }
        if let Some(notifier) = &self.slack.to_plain() {
            notifier
                .send_reqwest(text)
                .await
                .context("Failed to send Slack notification")?;
        }
        if let Some(notifier) = self.telegram.to_plain() {
            notifier
                .send_reqwest(text)
                .await
                .context("Failed to send Telegram notification")?;
        }
        if let Some(notifier) = &self.webhook.to_plain() {
            notifier
                .send_reqwest(text)
                .await
                .context("Failed to send webhook notification")?;
        }
        Ok(())
    }
}

#[derive(clap::Args)]
pub struct Matrix {
    #[arg(
        long,
        env,
        value_hint = clap::ValueHint::Url,
        value_name = "URL",
        requires = "notification_matrix_room_id",
        help_heading = "Notification Options",
    )]
    pub notification_matrix_homeserver: Option<Url>,

    #[arg(
        long,
        env,
        value_hint = clap::ValueHint::Other,
        value_name = "ROOM_ID",
        requires = "notification_matrix_access_token",
        help_heading = "Notification Options",
    )]
    pub notification_matrix_room_id: Option<String>,

    #[arg(
        long,
        env,
        hide_env_values = true,
        value_hint = clap::ValueHint::Other,
        value_name = "ACCESS_TOKEN",
        requires = "notification_matrix_homeserver",
        help_heading = "Notification Options",
    )]
    pub notification_matrix_access_token: Option<String>,
}
impl Matrix {
    #[must_use]
    pub fn to_plain(&self) -> Option<crate::Matrix> {
        if let (Some(homeserver), Some(room_id), Some(access_token)) = (
            &self.notification_matrix_homeserver,
            &self.notification_matrix_room_id,
            &self.notification_matrix_access_token,
        ) {
            Some(crate::Matrix {
                homeserver: homeserver.to_owned(),
                room_id: room_id.to_owned(),
                access_token: access_token.to_owned(),
            })
        } else {
            None
        }
    }
}

#[derive(clap::Args)]
pub struct Slack {
    /// Slack Incoming Webhook URL.
    ///
    /// See documentation: <https://api.slack.com/messaging/webhooks#getting_started>
    #[arg(
        long,
        env,
        hide_env_values = true,
        value_hint = clap::ValueHint::Url,
        value_name = "URL",
        help_heading = "Notification Options",
    )]
    pub notification_slack_webhook: Option<Url>,
}
impl Slack {
    #[must_use]
    pub fn to_plain(&self) -> Option<crate::Slack> {
        self.notification_slack_webhook
            .clone()
            .map(|webhook| crate::Slack { webhook })
    }
}

#[derive(clap::Args)]
pub struct Telegram {
    /// Bot Token from `@BotFather` in Telegram
    #[arg(
        long,
        env,
        hide_env_values = true,
        value_hint = clap::ValueHint::Other,
        value_name = "BOT_TOKEN",
        requires = "notification_telegram_target_chat",
        help_heading = "Notification Options",
    )]
    pub notification_telegram_bot_token: Option<String>,

    /// Chat/User ID or Chat/Channel Username.
    ///
    /// Make sure the bot is added to the chat/channel.
    #[arg(
        long,
        env,
        value_hint = clap::ValueHint::Other,
        value_name = "ID/USERNAME",
        requires = "notification_telegram_bot_token",
        help_heading = "Notification Options",
    )]
    pub notification_telegram_target_chat: Option<crate::TelegramTargetChat>,

    #[arg(
        long,
        env,
        requires = "notification_telegram_bot_token",
        help_heading = "Notification Options"
    )]
    pub notification_telegram_disable_web_page_preview: bool,

    /// Sends the message silently.
    ///
    /// Users will receive a notification with no sound.
    #[arg(
        long,
        env,
        requires = "notification_telegram_bot_token",
        help_heading = "Notification Options"
    )]
    pub notification_telegram_silent: bool,
}
impl Telegram {
    #[must_use]
    pub fn to_plain(&self) -> Option<crate::Telegram> {
        if let (Some(bot_token), Some(target_chat)) = (
            &self.notification_telegram_bot_token,
            &self.notification_telegram_target_chat,
        ) {
            Some(crate::Telegram {
                bot_token: bot_token.to_owned(),
                target_chat: target_chat.clone(),
                disable_web_page_preview: self.notification_telegram_disable_web_page_preview,
                disable_notification: self.notification_telegram_silent,
                parse_mode: None,
            })
        } else {
            None
        }
    }
}

#[derive(clap::Args)]
pub struct Webhook {
    /// Send a POST request to the given URL.
    ///
    /// The body will contain the text of the notification.
    #[arg(
        long,
        env,
        hide_env_values = true,
        value_hint = clap::ValueHint::Url,
        value_name = "URL",
        help_heading = "Notification Options",
    )]
    pub notification_webhook: Option<Url>,
}
impl Webhook {
    #[must_use]
    pub fn to_plain(&self) -> Option<crate::Webhook> {
        self.notification_webhook
            .clone()
            .map(|webhook| crate::Webhook { webhook })
    }
}

#[test]
fn verify_command() {
    use clap::{CommandFactory as _, Parser};

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        pub notifications: Args,
    }

    Cli::command().debug_assert();
}
