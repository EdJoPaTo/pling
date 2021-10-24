use pling::Webhook;

fn main() {
    let webhook = Webhook::from_env().expect("WEBHOOK_URL is not defined");

    webhook
        .send_sync("test pling please ignore")
        .expect("failed to send pling");
}
