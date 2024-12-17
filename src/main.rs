use regex::Regex;
use reqwest::{self, Proxy};
use scraper::{Html, Selector};
use std::env;

async fn get_bus_info(stop_cd_from: &str, stop_cd_to: &str, proxy_url: Option<&str>) {
    let client = if let Some(proxy) = proxy_url {
        reqwest::Client::builder()
            .proxy(Proxy::all(proxy).unwrap())
            .build()
            .unwrap()
    } else {
        reqwest::Client::new()
    };

    let url = format!(
        "https://mc.bus-vision.jp/ibako/view/approach.html?stopCdFrom={}&stopCdTo={}",
        stop_cd_from, stop_cd_to
    );
    let response = client.get(&url).send().await.unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let document = Html::parse_document(&body);
        let selector = Selector::parse("body").unwrap();
        let info = match document.select(&selector).next() {
            Some(element) => element.text().collect::<Vec<_>>().join(""),
            None => {
                println!("情報が見つかりませんでした。");
                return;
            }
        };

        let location_pattern = Regex::new(r"出発⇒到着").unwrap();
        if let Some(location_match) = location_pattern.find(&info) {
            println!("出発⇒到着: {}", location_match.as_str());
        }

        let patterns = vec![
            ("到着まで", Regex::new(r"あと(\d+)分で到着予定").unwrap()),
            (
                "出発⇒到着",
                Regex::new(r"(\d{2}:\d{2})発.*?(\d{2}:\d{2})着").unwrap(),
            ),
            ("遅れ", Regex::new(r"約(\d+)分遅れ").unwrap()),
            (
                "乗車予定",
                Regex::new(r"乗定刻(\d{2}:\d{2})（予測(\d{2}:\d{2})）").unwrap(),
            ),
            (
                "降車予定",
                Regex::new(r"降定刻(\d{2}:\d{2})（予測(\d{2}:\d{2})）").unwrap(),
            ),
        ];

        for (key, pattern) in patterns {
            if let Some(captures) = pattern.captures(&info) {
                let groups: Vec<String> = captures
                    .iter()
                    .skip(1)
                    .map(|m| {
                        let mut text = m.unwrap().as_str().to_string();
                        if key == "到着まで" || key == "遅れ" {
                            text.push_str("分");
                        }
                        text
                    })
                    .collect();
                println!("{}: {}", key, groups.join(" ⇒ "));
            }
        }
    } else {
        println!(
            "ページの取得に失敗しました。ステータスコード: {}",
            response.status()
        );
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("使用方法: cargo run <kosen|station> [proxy_url]");
    } else {
        let (stop_cd_from, stop_cd_to) = if args[1] == "kosen" {
            ("69", "76")
        } else {
            ("76", "69")
        };
        let proxy_url = if args.len() == 3 {
            Some(args[2].as_str())
        } else {
            None
        };
        get_bus_info(stop_cd_from, stop_cd_to, proxy_url).await;
    }
}
