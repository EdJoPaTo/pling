mod parse_mode;
mod target_chat;

pub use parse_mode::ParseMode;
pub use target_chat::TargetChat;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../../docs/telegram.md")]
pub struct Telegram {
    pub bot_token: String,
    pub target_chat: TargetChat,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::serde_helper::is_default")
    )]
    pub disable_web_page_preview: bool,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::serde_helper::is_default")
    )]
    pub disable_notification: bool,
}

impl From<Telegram> for crate::Notification {
    fn from(tg: Telegram) -> Self {
        Self::Telegram(tg)
    }
}

impl Telegram {
    #[must_use]
    pub const fn new(bot_token: String, target_chat: TargetChat) -> Self {
        Self {
            bot_token,
            target_chat,
            disable_web_page_preview: false,
            disable_notification: false,
        }
    }

    #[must_use]
    /// Loads the Telegram notification config from environment variables.
    /// The following variables are used:
    /// - `TELEGRAM_BOT_TOKEN`
    /// - `TELEGRAM_TARGET_CHAT`
    /// - `TELEGRAM_DISABLE_WEB_PAGE_PREVIEW`
    /// - `TELEGRAM_DISABLE_NOTIFICATION`
    ///
    /// When `TELEGRAM_BOT_TOKEN` or `TELEGRAM_TARGET_CHAT` are unset None is returned.
    pub fn from_env() -> Option<Self> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").ok()?;
        let target_chat = std::env::var("TELEGRAM_TARGET_CHAT")
            .ok()
            .and_then(|s| s.parse().ok())?;
        let disable_web_page_preview =
            std::env::var_os("TELEGRAM_DISABLE_WEB_PAGE_PREVIEW").is_some();
        let disable_notification = std::env::var_os("TELEGRAM_DISABLE_NOTIFICATION").is_some();

        Some(Self {
            bot_token,
            target_chat,
            disable_web_page_preview,
            disable_notification,
        })
    }

    #[must_use]
    fn generate_url(&self) -> String {
        format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token)
    }

    #[must_use]
    fn base_form(
        &self,
        parse_mode: Option<ParseMode>,
        disable_web_page_preview: bool,
        disable_notification: bool,
    ) -> Vec<(&str, &str)> {
        let mut form = vec![
            (
                "disable_web_page_preview",
                str_bool(disable_web_page_preview || self.disable_web_page_preview),
            ),
            (
                "disable_notification",
                str_bool(disable_notification || self.disable_notification),
            ),
        ];
        if let Some(parse_mode) = parse_mode {
            form.push(("parse_mode", parse_mode.to_str()));
        }
        form
    }

    #[cfg(feature = "http-sync")]
    /// Send a Telegram notification synchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    pub fn send_sync(
        &self,
        text: &str,
        parse_mode: Option<ParseMode>,
        disable_web_page_preview: bool,
        disable_notification: bool,
    ) -> Result<(), ureq::Error> {
        let mut form = self.base_form(parse_mode, disable_web_page_preview, disable_notification);
        let target_chat = self.target_chat.to_string();
        form.push(("chat_id", target_chat.as_ref()));
        form.push(("text", text));

        ureq::post(&self.generate_url())
            .set("User-Agent", crate::USER_AGENT)
            .send_form(&form)?;
        Ok(())
    }

    #[cfg(feature = "http-async")]
    /// Send a Telegram notification asynchronously.
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    pub async fn send_async(
        &self,
        text: &str,
        parse_mode: Option<ParseMode>,
        disable_web_page_preview: bool,
        disable_notification: bool,
    ) -> Result<(), reqwest::Error> {
        let mut form = self.base_form(parse_mode, disable_web_page_preview, disable_notification);
        let target_chat = self.target_chat.to_string();
        form.push(("chat_id", target_chat.as_ref()));
        form.push(("text", text));

        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(self.generate_url())
            .form(&form)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[must_use]
const fn str_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[test]
fn can_serde_parse_telegram() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
    };
    crate::test_helper::can_serde_parse(&telegram);
}

#[test]
fn url_correct() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
    };
    let url = telegram.generate_url();
    assert_eq!(url, "https://api.telegram.org/bot123:ABC/sendMessage");
}

#[test]
fn base_form_minimal() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
    };
    let form = telegram.base_form(None, false, false);
    dbg!(&form);
    assert_eq!(
        form,
        [
            ("disable_web_page_preview", "false"),
            ("disable_notification", "false"),
        ]
    );
}

#[test]
fn base_form_with_parse_mode() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
    };
    let form = telegram.base_form(Some(ParseMode::Markdown), false, false);
    dbg!(&form);
    assert_eq!(
        form,
        [
            ("disable_web_page_preview", "false"),
            ("disable_notification", "false"),
            ("parse_mode", "Markdown"),
        ]
    );
}

#[test]
fn base_form_disable_from_struct() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: true,
        disable_notification: false,
    };
    let form = telegram.base_form(None, false, false);
    dbg!(&form);
    assert_eq!(
        form,
        [
            ("disable_web_page_preview", "true"),
            ("disable_notification", "false"),
        ]
    );
}

#[test]
fn base_form_disable_from_method() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
    };
    let form = telegram.base_form(None, true, false);
    dbg!(&form);
    assert_eq!(
        form,
        [
            ("disable_web_page_preview", "true"),
            ("disable_notification", "false"),
        ]
    );
}
