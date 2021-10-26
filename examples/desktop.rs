use pling::Desktop;

fn main() {
    let desktop = Desktop::from_env().expect("PLING_DESKTOP_ENABLED is not defined");

    desktop
        .send_sync("test pling please ignore")
        .expect("failed to send pling");
}
