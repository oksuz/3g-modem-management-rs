use rand::seq::IndexedRandom;
use std::{error::Error, sync::Arc, time::Instant};

use dotenvy::dotenv;
use modem_scanner::{
    api::get_active_numbers,
    at_command::get_iccid,
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

pub fn get_random_number(numbers: &[String]) -> Option<&String> {
    let mut rng = rand::rng();

    numbers.choose(&mut rng)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let modems = Arc::new(DeviceMap::new());
    let ports = scanner::scan_ports();

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

    let numbers = get_active_numbers().await.expect("cannot fetch numbers");

    for modem in modems.iter() {
        let number = get_random_number(&numbers).unwrap();
        let iccid = get_iccid(&modem.port).await.ok_or("cannot read iccid")?;

        sms::send_sms(&modem.port, &number, Some(iccid.clone()))
            .await
            .ok_or("Cannot send the sms")?;

        println!(
            "the sms has been sent to {}, for iccID: {} on port {}",
            &number, &iccid, &modem.port
        )
    }

    Ok(())
}
