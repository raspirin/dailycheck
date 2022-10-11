#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct ResponseInfo {
    // 弱智西电垃圾命名隔空污染我代码
    e: i32,
    m: String,
    d: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        // 微信UA
        .user_agent("Mozilla/5.0 (Linux; Android 12; V2171A Build/SP1A.210812.003; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/86.0.4240.99 XWEB/4317 MMWEBSDK/20220805 Mobile Safari/537.36 MMWEBID/3736 MicroMessenger/8.0.27.2220(0x28001B59) WeChat/arm64").build().unwrap();

    // 从环境变量中取用户名和密码，便于使用 GitHub Actions
    let username = env::var("CHECKUP_USERNAME").unwrap();
    let password = env::var("CHECKUP_PASSWORD").unwrap();
    let login_post = [("username", &username), ("password", &password)];
    let login_resp = client
        .post("https://xxcapp.xidian.edu.cn/uc/wap/login/check")
        .form(&login_post)
        .send()
        .await?;
    let login_info = login_resp.json::<ResponseInfo>().await?;
    if login_info.e != 0 {
        panic!("登陆时：{}", login_info.m);
    }

    let geo_api_info = r#"{"type":"complete","position":{"Q":34.125585394966,"R":108.83212402343798,"lng":108.832124,"lat":34.125585},"location_type":"html5","message":"Get ipLocation failed.Get geolocation success.Convert Success.Get address success.","accuracy":35,"isConverted":true,"status":1,"addressComponent":{"citycode":"029","adcode":"610116","businessAreas":[],"neighborhoodType":"","neighborhood":"","building":"","buildingType":"","street":"雷甘路","streetNumber":"266#","country":"中国","province":"陕西省","city":"西安市","district":"长安区","township":"兴隆街道"},"formattedAddress":"陕西省西安市长安区兴隆街道西安电子科技大学长安校区西二楼B西安电子科技大学南校区","roads":[],"crosses":[],"pois":[],"info":"SUCCESS"}"#;
    let checkup_post = [
        ("sfzx", "1"),
        ("tw", "1"),
        ("area", "陕西省 西安市 长安区"),
        ("city", "西安市"),
        ("province", "陕西省"),
        (
            "address",
            "陕西省西安市长安区兴隆街道西安电子科技大学长安校区西二楼B西安电子科技大学南校区",
        ),
        ("geo_api_info", geo_api_info),
        ("sfcyglq", "0"),
        ("sfyzz", "0"),
        ("qtqk", ""),
        ("ymtys", "0"),
    ];
    let checkup_resp = client
        .post("https://xxcapp.xidian.edu.cn/xisuncov/wap/open-report/save")
        .form(&checkup_post)
        .send()
        .await?;
    let checkup_info = checkup_resp.json::<ResponseInfo>().await?;
    if checkup_info.e != 0 {
        panic!("填报时：{}", checkup_info.m);
    }

    Ok(())
}
