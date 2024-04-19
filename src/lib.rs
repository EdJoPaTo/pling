/*!
Send notifications via Slack, Telegram, ...

The name of this Rust crate is inspired by the notification arrival sound.
*/

#![cfg_attr(not(any(feature = "reqwest", feature = "ureq")), allow(dead_code))]

#[cfg(feature = "clap")]
pub mod clap;

mod matrix;
mod slack;
mod telegram;
mod webhook;

pub use crate::matrix::Matrix;
pub use crate::slack::Slack;
pub use crate::telegram::{
    ParseMode as TelegramParseMode, TargetChat as TelegramTargetChat, Telegram,
};
pub use crate::webhook::Webhook;

pub(crate) const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);
