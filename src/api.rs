use regex::Regex;
use reqwest::StatusCode;
use std::{env, sync::LazyLock, thread, time::Duration};

static TR_MSISDN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\+90\d{10}").expect("valid regex TR_MSISDN_RE"));

static SENDER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"sender":"(\+90\d{10})""#).expect("valid regex for SENDER_RE"));

static MESSAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"iccid is:'(\w+)'"#).expect("valid regex for MESSAGE_RE"));

static SMS_READ_MAX_RETRY: LazyLock<usize> = LazyLock::new(|| {
    env::var("SMS_READ_MAX_RETRY")
        .ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(15)
});

static SMS_READ_SLEEP_DURATION: LazyLock<usize> = LazyLock::new(|| {
    env::var("SMS_READ_SLEEP_DURATION")
        .ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(3)
});

static SMS_READ_API: LazyLock<String> = LazyLock::new(|| env::var("SMS_READ_API").unwrap());

static ACTIVE_SIMS_API: LazyLock<String> = LazyLock::new(|| env::var("ACTIVE_SIMS_API").unwrap());

static MSISDN_REGISTER_API: LazyLock<String> =
    LazyLock::new(|| env::var("MSISDN_REGISTER_API").unwrap());

pub async fn get_active_msisdns() -> Option<Vec<String>> {
    let handle = || {
        let content = reqwest::blocking::get(&*ACTIVE_SIMS_API)
            .and_then(|r| r.text())
            .map_err(|e| {
                eprintln!("error on exporting active sim {:?}", e);
                e
            })
            .ok()?;

        Some(
            TR_MSISDN_RE
                .find_iter(&content)
                .map(|m| m.as_str().to_owned())
                .collect(),
        )
    };

    tokio::task::spawn_blocking(handle).await.unwrap()
}

pub async fn read_remote_sms(phone_number: &str, icc_id: &str) -> Option<String> {
    let phone_number = phone_number.to_owned();
    let icc_id = icc_id.to_owned();
    let handle = move || {
        let url = format!("{}?phone_number={}&limit=1", *SMS_READ_API, phone_number);

        for _ in 0..*SMS_READ_MAX_RETRY {
            let response = reqwest::blocking::get(&url)
                .and_then(|r| r.text())
                .map_err(|e| {
                    eprintln!("cannot get response {:?}", e);
                    e
                })
                .ok()?;

            let found = MESSAGE_RE
                .captures_iter(&response)
                .any(|cap| cap.get(1).map_or(false, |m| m.as_str() == icc_id));

            if found {
                let msisdn: Option<String> = SENDER_RE
                    .captures_iter(&response)
                    .find_map(|capture| capture.get(1).map(|c| c.as_str().to_owned()));

                return msisdn;
            }

            thread::sleep(Duration::from_secs(*SMS_READ_SLEEP_DURATION as u64));
        }

        None
    };

    tokio::task::spawn_blocking(handle).await.unwrap()
}

pub async fn register_msisdn(msisdn: &str, icc_id: &str) -> Option<(String, StatusCode)> {
    let msisd = msisdn.to_owned();
    let icc_id = icc_id.to_owned();
    let handle = move || {
        let payload = format!(r#"{{"msisdn": "{}", "icc_id": "{}"}}"#, &msisd, &icc_id);
        println!("sending the payload: {}", payload);
        let client = reqwest::blocking::Client::new();
        client
            .post(&*MSISDN_REGISTER_API)
            .body(payload)
            .header("content-type", "application/json")
            .send()
            .map_err(|e| {
                eprintln!("error on registering the gsm number: {:?}", e);
                e
            })
            .and_then(|response| {
                let status = response.status();
                response.text().map(|r| (r, status))
            })
            .ok()
    };

    tokio::task::spawn_blocking(handle).await.unwrap()
}
