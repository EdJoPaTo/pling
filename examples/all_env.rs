use pling::Notifier;

fn main() {
    let notifiers = Notifier::from_env();
    for notifier in notifiers {
        notifier
            .send_sync("test pling please ignore")
            .expect("failed to send pling");
    }
}
