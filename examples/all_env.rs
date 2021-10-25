use pling::Notification;

fn main() {
    let all = Notification::from_env();

    for notification in all {
        notification
            .send_sync("test pling please ignore")
            .expect("failed to send pling");
    }
}
