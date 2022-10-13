fn main() {
    let client = dailycheck::CheckupClient::new();

    if let Err(v) = client.login_post() {
        panic!("{}", v)
    }

    if let Err(v) = client.checkup_post() {
        panic!("{}", v)
    }
}
