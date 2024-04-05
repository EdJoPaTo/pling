mod target_chat;

pub use target_chat::TargetChat;

/// Telegram Notification
///
/// Documentation: <https://core.telegram.org/bots/api#sendmessage>
#[derive(Clone, PartialEq, Eq)]
pub struct Telegram {
    /// Bot Token from `@BotFather` in Telegram
    pub bot_token: String,

    pub target_chat: TargetChat,
    pub disable_web_page_preview: bool,
    pub disable_notification: bool,
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
    #[allow(clippy::result_large_err)]
    #[cfg(feature = "ureq")]
    pub fn send_ureq(&self, text: &str) -> Result<(), ureq::Error> {
        let mut form = self.base_form();
        let target_chat = self.target_chat.to_string();
        form.push(("chat_id", target_chat.as_ref()));
        form.push(("text", text));

        ureq::post(&generate_url(&self.bot_token))
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
    pub async fn send_reqwest(&self, text: &str) -> reqwest::Result<()> {
        let mut form = self.base_form();
        let target_chat = self.target_chat.to_string();
        form.push(("chat_id", target_chat.as_ref()));
        form.push(("text", text));

        reqwest::ClientBuilder::new()
            .user_agent(crate::USER_AGENT)
            .build()?
            .post(generate_url(&self.bot_token))
            .form(&form)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[must_use]
fn generate_url(bot_token: &str) -> String {
    format!("https://api.telegram.org/bot{bot_token}/sendMessage")
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
        bot_token: "123:ABC".to_owned(),
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
