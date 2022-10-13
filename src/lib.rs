use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::i32;
use std::vec;

// 服务器返回的json结构
#[derive(Deserialize)]
pub struct ResponseInfo {
    // 弱智西电垃圾命名隔空污染我代码
    pub e: i32,
    pub m: String,
    pub d: HashMap<String, String>,
}

pub struct CheckupClient {
    client: Client,
    username: String,
    password: String,
    campus: i32,
}

impl CheckupClient {
    pub fn new() -> Self {
        let client = Client::builder()
        .cookie_store(true)
        // 微信UA
        .user_agent("Mozilla/5.0 (Linux; Android 12; V2171A Build/SP1A.210812.003; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/86.0.4240.99 XWEB/4317 MMWEBSDK/20220805 Mobile Safari/537.36 MMWEBID/3736 MicroMessenger/8.0.27.2220(0x28001B59) WeChat/arm64")
        .build()
        .unwrap();
        let username = get_env_var("CHECKUP_USERNAME");
        let password = get_env_var("CHECKUP_PASSWORD");
        let campus = get_campus();
        CheckupClient {
            client,
            username,
            password,
            campus,
        }
    }

    fn post(&self, url: &str, body: &Vec<(&str, &str)>) -> Result<(), String> {
        let resp = self
            .client
            .post(url)
            .form(body)
            .send()
            .unwrap()
            .json::<ResponseInfo>()
            .unwrap();
        if resp.e != 0 {
            return Err(resp.m);
        }
        Ok(())
    }

    pub fn login_post(&self) -> Result<(), String> {
        let url = "https://xxcapp.xidian.edu.cn/uc/wap/login/check";
        let username = self.username.as_str();
        let password = self.password.as_str();
        let body = vec![("username", username), ("password", password)];
        match self.post(url, &body) {
            Ok(_) => Ok(()),
            Err(v) => Err(format!("登录时发生错误：{}", v)),
        }
    }

    pub fn checkup_post(&self) -> Result<(), String> {
        let url = "https://xxcapp.xidian.edu.cn/xisuncov/wap/open-report/save";
        let body = get_checkup_post(self.campus);
        match self.post(url, &body) {
            Ok(_) => Ok(()),
            Err(v) => Err(format!("填报时发生错误：{}", v)),
        }
    }
}

fn get_env_var(var: &str) -> String {
    match env::var(var) {
        Ok(v) => v,
        Err(_) => panic!("初始化信息时发生错误：未获取到 {}", var),
    }
}

// 为了不破坏原有的Fork对此环境变量进行单独处理
fn get_campus() -> i32 {
    match env::var("CHECKUP_CAMPUS") {
        Ok(mut v) => {
            if v.is_empty() {
                v.push('0');
            }
            v.as_str().parse::<i32>().unwrap()
        }
        Err(_) => 0,
    }
}

fn get_checkup_post(campus: i32) -> Vec<(&'static str, &'static str)> {
    match campus {
        // 南校区
        0 => {
            let geo_api_info = r#"{"type":"complete","position":{"Q":34.125585394966,"R":108.83212402343798,"lng":108.832124,"lat":34.125585},"location_type":"html5","message":"Get ipLocation failed.Get geolocation success.Convert Success.Get address success.","accuracy":35,"isConverted":true,"status":1,"addressComponent":{"citycode":"029","adcode":"610116","businessAreas":[],"neighborhoodType":"","neighborhood":"","building":"","buildingType":"","street":"雷甘路","streetNumber":"266#","country":"中国","province":"陕西省","city":"西安市","district":"长安区","township":"兴隆街道"},"formattedAddress":"陕西省西安市长安区兴隆街道西安电子科技大学长安校区西二楼B西安电子科技大学南校区","roads":[],"crosses":[],"pois":[],"info":"SUCCESS"}"#;
            vec![
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
            ]
        }
        // 北校区
        1 => {
            let geo_api_info = r#"{"type":"complete","position":{"Q":34.231309950087,"R":108.917966851129,"lng":108.917967,"lat":34.23131},"location_type":"html5","message":"Get geolocation success.Convert Success.Get address success.","accuracy":50.080511185456906,"isConverted":true,"status":1,"addressComponent":{"citycode":"029","adcode":"610113","businessAreas":[],"neighborhoodType":"","neighborhood":"","building":"","buildingType":"","street":"二环南路西段","streetNumber":"136号","country":"中国","province":"陕西省","city":"西安市","district":"雁塔区","towncode":"610113004000","township":"电子城街道"},"formattedAddress":"陕西省西安市雁塔区电子城街道西安路西安电子科技大学北校区","roads":[],"crosses":[],"pois":[],"info":"SUCCESS"}"#;
            vec![
                ("sfzx", "1"),
                ("tw", "1"),
                ("area", "陕西省 西安市 雁塔区"),
                ("city", "西安市"),
                ("province", "陕西省"),
                (
                    "address",
                    "陕西省西安市雁塔区电子城街道西安路西安电子科技大学北校区",
                ),
                ("geo_api_info", geo_api_info),
                ("sfcyglq", "0"),
                ("sfyzz", "0"),
                ("qtqk", ""),
                ("ymtys", "0"),
            ]
        }
        _ => panic!("初始化信息时错误：错误的校区信息"),
    }
}
