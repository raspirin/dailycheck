fn main() {
    let client = dailycheck::CheckupClient::new();

    if let Err(v) = client.login_post() {
        panic!("{}", v)
    }

    if let Err(v) = client.checkup_post() {
        match v.as_str() {
            "填报时发生错误：您已上报过" => println!("{}", v),
            _ => panic!("{}", v)
        }
    }
}
