use std::{sync::Arc, time::Instant};

use modem_scanner::{
    device_map::{DeviceMap, ModemInfo},
    scanner, sms,
};

fn add_modem(port: &str, imei: Option<String>, modems: Arc<DeviceMap>) {
    if let Some(imei) = imei {
        modems.insert(
            imei.clone(),
            ModemInfo {
                port: port.into(),
                imei: imei,
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
        match scanner::probe_port(&p).await {
            Some((port, imei)) => {
                add_modem(port, imei, modems.clone());
            }
            None => {
                println!("skipping port {}", p);
            }
        };
    }

    for modem in modems.iter() {
        let sms_result = sms::send_sms(
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
