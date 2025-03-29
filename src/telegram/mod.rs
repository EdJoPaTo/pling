pub use self::parse_mode::ParseMode;
pub use self::target_chat::TargetChat;

mod parse_mode;
mod target_chat;

/// Telegram Notification
///
/// Documentation: <https://core.telegram.org/bots/api#sendmessage>
#[derive(Clone, PartialEq, Eq)]
pub struct Telegram {
    /// Bot Token from `@BotFather` in Telegram
    pub bot_token: String,

    pub target_chat: TargetChat,

    // optional
    pub disable_web_page_preview: bool,
    pub disable_notification: bool,
    pub parse_mode: Option<ParseMode>,
}

impl Telegram {
    #[must_use]
    pub const fn new(bot_token: String, target_chat: TargetChat) -> Self {
        Self {
            bot_token,
            target_chat,
            disable_web_page_preview: false,
            disable_notification: false,
            parse_mode: None,
        }
    }

    #[must_use]
    fn base_form(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if self.disable_web_page_preview {
            result.push(("disable_web_page_preview", "true"));
        }
        if self.disable_notification {
            result.push(("disable_notification", "true"));
        }
        if let Some(parsemode) = self.parse_mode {
            result.push(("parse_mode", parsemode.to_str()));
        }
        result
    }

    /// Send a Telegram notification via [`ureq`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    #[allow(clippy::result_large_err)]
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> Result<(), ureq::Error> {
        let mut form = self.base_form();
        let chat_id = self.target_chat.to_chat_id();
        form.push(("chat_id", &chat_id));
        form.push(("text", text));

        ureq::post(&generate_url(&self.bot_token))
            .header(ureq::http::header::USER_AGENT, crate::USER_AGENT_UREQ)
            .send_form(form)?;
        Ok(())
    }

    /// Send a Telegram notification via [`reqwest`].
    ///
    /// # Errors
    ///
    /// This method errors when the request could not be send or the not be handled by the Telegram API.
    #[cfg(feature = "reqwest")]
    pub async fn send_reqwest(&self, text: &str) -> reqwest::Result<()> {
        let mut form = self.base_form();
        let chat_id = self.target_chat.to_chat_id();
        form.push(("chat_id", &chat_id));
        form.push(("text", text));

        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT_REQWEST)
            .build()?
            .post(generate_url(&self.bot_token))
            .form(&form)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(reqwest::Error::without_url)?;
        Ok(())
    }
}

#[must_use]
fn generate_url(bot_token: &str) -> String {
    format!("https://api.telegram.org/bot{bot_token}/sendMessage")
}

#[test]
fn url_correct() {
    let url = generate_url("123:ABC");
    assert_eq!(url, "https://api.telegram.org/bot123:ABC/sendMessage");
}

#[test]
fn base_form_minimal() {
    let telegram = Telegram {
        bot_token: "123:ABC".to_owned(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
        parse_mode: None,
    };
    let form = telegram.base_form();
    dbg!(&form);
    assert_eq!(form, []);
}

#[test]
fn base_form_disable_preview() {
    let telegram = Telegram {
        bot_token: "123:ABC".to_owned(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: true,
        disable_notification: false,
        parse_mode: None,
    };
    let form = telegram.base_form();
    dbg!(&form);
    assert_eq!(form, [("disable_web_page_preview", "true"),]);
}

#[test]
fn base_form_parse_mode() {
    let telegram = Telegram {
        bot_token: "123:ABC".to_owned(),
        target_chat: TargetChat::Id(1234),
        disable_web_page_preview: false,
        disable_notification: false,
        parse_mode: Some(ParseMode::HTML),
    };
    let form = telegram.base_form();
    dbg!(&form);
    assert_eq!(form, [("parse_mode", "HTML"),]);
}
