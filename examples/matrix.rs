use pling::Matrix;

fn main() {
    let matrix = Matrix::from_env().expect("MATRIX environment is not fully defined");

    matrix
        .send_ureq("test pling please ignore")
        .expect("failed to send pling");
}
