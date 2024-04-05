mod target_chat;

pub use target_chat::TargetChat;

#[derive(Clone, PartialEq, Eq)]
#[doc = include_str!("../../docs/telegram.md")]
pub struct Telegram {
    pub bot_token: String,
    pub target_chat: TargetChat,

    pub disable_web_page_preview: bool,

    pub disable_notification: bool,
}

impl From<Telegram> for crate::Notifier {
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

    /// Loads the Telegram config from environment variables.
    /// The following variables are used:
    /// - `TELEGRAM_BOT_TOKEN`
    /// - `TELEGRAM_TARGET_CHAT`
    /// - `TELEGRAM_DISABLE_WEB_PAGE_PREVIEW`
    /// - `TELEGRAM_DISABLE_NOTIFICATION`
    ///
    /// When `TELEGRAM_BOT_TOKEN` or `TELEGRAM_TARGET_CHAT` are unset None is returned.
    #[must_use]
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
    fn base_form(&self) -> Vec<(&str, &str)> {
        vec![
            (
                "disable_web_page_preview",
                str_bool(self.disable_web_page_preview),
            ),
            ("disable_notification", str_bool(self.disable_notification)),
        ]
    }

    /// Send a Telegram notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> Result<(), ureq::Error> {
        let mut form = self.base_form();
        let target_chat = self.target_chat.to_string();
        form.push(("chat_id", target_chat.as_ref()));
        form.push(("text", text));

        ureq::post(&self.generate_url())
            .set("User-Agent", crate::USER_AGENT)
            .send_form(&form)?;
        Ok(())
    }

    /// Send a Telegram notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> Result<(), reqwest::Error> {
        let mut form = self.base_form();
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
    let form = telegram.base_form();
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
fn base_form_disable() {
    let telegram = Telegram {
        bot_token: "123:ABC".into(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: true,
        disable_notification: false,
    };
    let form = telegram.base_form();
    dbg!(&form);
    assert_eq!(
        form,
        [
            ("disable_web_page_preview", "true"),
            ("disable_notification", "false"),
        ]
    );
}
