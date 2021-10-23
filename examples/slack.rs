use pling::Slack;

fn main() {
    let slack = Slack::from_env().expect("SLACK_HOOK is not defined");

    slack
        .send_sync("test pling please ignore")
        .expect("failed to send pling");
}
