use regex::Regex;
use std::{env, sync::LazyLock};

static TR_MSISDN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\+90\d{10}").expect("valid regex"));

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
