#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        // 微信UA
        .user_agent("Mozilla/5.0 (Linux; Android 12; V2171A Build/SP1A.210812.003; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/86.0.4240.99 XWEB/4317 MMWEBSDK/20220805 Mobile Safari/537.36 MMWEBID/3736 MicroMessenger/8.0.27.2220(0x28001B59) WeChat/arm64")
        .build()
        .unwrap();

    // 从环境变量中取用户名和密码，便于使用 GitHub Actions
    let username = dailycheck::get_env_var("CHECKUP_USERNAME");
    let password = dailycheck::get_env_var("CHECKUP_PASSWORD");
    let campus = dailycheck::get_campus();

    let login_post = [("username", &username), ("password", &password)];
    let login_resp = client
        .post("https://xxcapp.xidian.edu.cn/uc/wap/login/check")
        .form(&login_post)
        .send()
        .await?;
    let login_info = login_resp.json::<dailycheck::ResponseInfo>().await?;
    if login_info.e != 0 {
        panic!("登录时发生错误：{}", login_info.m);
    }

    let checkup_post = dailycheck::get_checkup_post(campus);
    let checkup_resp = client
        .post("https://xxcapp.xidian.edu.cn/xisuncov/wap/open-report/save")
        .form(&checkup_post)
        .send()
        .await?;
    let checkup_info = checkup_resp.json::<dailycheck::ResponseInfo>().await?;
    if checkup_info.e != 0 {
        if checkup_info.m != "您已上报过" {
            panic!("填报时发生错误：{}", checkup_info.m);
        }
        println!("填报时发生错误：{}", checkup_info.m);
    }

    Ok(())
}
