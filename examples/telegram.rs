use pling::Telegram;

fn main() {
    let telegram =
        Telegram::from_env().expect("TELEGRAM_BOT_TOKEN or TELEGRAM_TARGET_CHAT are not defined");

    telegram
        .send_sync("test pling please ignore")
        .expect("failed to send pling");
}
