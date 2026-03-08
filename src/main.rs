use std::{sync::Arc, time::Instant};

use crate::{
    device_map::{DeviceMap, ModemInfo},
    scanner::probe_port,
    sms::send_sms,
};

mod at_command;
mod device_map;
mod parser;
mod scanner;
mod sms;

fn add_modem(port: &str, icc_id: Option<String>, modems: Arc<DeviceMap>) {
    if let Some(icc_id) = icc_id {
        modems.insert(
            icc_id.clone(),
            ModemInfo {
                port: port.into(),
                imei: icc_id,
                last_seen: Instant::now(),
            },
        );
    }
}

#[tokio::main]
async fn main() {
    let modems = Arc::new(DeviceMap::new());
    let ports = scanner::scan_ports().await;

    println!("found  ports {:?}", ports);

    for p in &ports {
        match probe_port(&p).await {
            Some((port, icc_id)) => {
                add_modem(port, icc_id, modems.clone());
            }
            None => {
                println!("skipping port {}", p);
            }
        };
    }

    for modem in modems.iter() {
        let sms_result = send_sms(
            &modem.port,
            "+90",
            Some("this message has been sent via a rust code!"),
        )
        .await;

        match sms_result {
            Some(_) => {
                println!("sms has been sent!");
            }
            None => {
                eprintln!("sms has not been sent");
            }
        }
    }
}
