use regex::Regex;
use std::{env, sync::LazyLock, thread, time::Duration};

static TR_MSISDN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\+90\d{10}").expect("valid regex TR_MSISDN_RE"));

static SENDER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"sender":"(\+90\d{10})""#).expect("valid regex for SENDER_RE"));

static MESSAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"iccid is:'(\w+)'"#).expect("valid regex for MESSAGE_RE"));

pub async fn get_active_numbers() -> Option<Vec<String>> {
    let handle = || {
        let url = env::var("ACTIVE_SIMS").ok()?;

        let content = reqwest::blocking::get(url)
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

pub async fn read_remote_sms(
    phone_number: &str,
    icc_id: &str,
    max_retry: Option<usize>,
) -> Option<String> {
    let phone_number = phone_number.to_string();
    let icc_id = icc_id.to_string();
    let max_retry = max_retry.unwrap_or(15);
    let handle = move || {
        let sms_read_api = env::var("SMS_READ_API").ok()?;
        let url = format!("{}?phone_number={}&limit=1", sms_read_api, phone_number);
        let mut current_try = 0;

        loop {
            if current_try > max_retry {
                break;
            }

            let response = reqwest::blocking::get(&url)
                .and_then(|r| r.text())
                .map_err(|e| {
                    eprintln!("cannot get response {:?}", e);
                    e
                })
                .ok()?;

            current_try += 1;

            let captured_messages: Option<Vec<String>> = MESSAGE_RE
                .captures_iter(&response)
                .map(|capture| capture.get(1).map(|c| c.as_str().to_owned()))
                .collect();

            let found = captured_messages
                .as_ref()
                .into_iter()
                .flatten()
                .any(|v| icc_id.eq(v));

            if found {
                let msisdn: Option<String> = SENDER_RE
                    .captures_iter(&response)
                    .find_map(|capture| capture.get(1).map(|c| c.as_str().to_owned()));

                return msisdn;
            }

            thread::sleep(Duration::from_secs(3));
        }

        None
    };

    tokio::task::spawn_blocking(handle).await.unwrap()
}
