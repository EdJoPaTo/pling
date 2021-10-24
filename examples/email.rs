use pling::Email;

fn main() {
    let email = Email::from_env().expect("EMAIL environment is not fully defined");

    email
        .send_sync("test pling please ignore")
        .expect("failed to send pling");
}
