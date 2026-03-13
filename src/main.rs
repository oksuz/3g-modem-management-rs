use dotenvy::dotenv;
use modem_scanner::{
    api::{get_active_numbers, read_remote_sms},
    at_command::get_iccid,
    device_map::{DeviceMap, ModemInfo},
    scanner, sms,
};
use rand::seq::IndexedRandom;
use std::{error::Error, sync::Arc, time::Instant};

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

        let Some(icc_id) = get_iccid(&modem.port).await else {
            eprintln!("cannot read iccid on port {}", modem.port);
            continue;
        };

        let sms_content = format!("iccid is:'{}'", icc_id.clone());
        let Some(()) = sms::send_sms(&modem.port, &number, Some(sms_content)).await else {
            eprintln!("cannot send the sms to number {}", &number);
            continue;
        };

        println!(
            "the sms has been sent to {}, for iccID: {} on port {}",
            &number, &icc_id, &modem.port
        );

        let Some(msisdn) = read_remote_sms(&number, &icc_id, None).await else {
            eprintln!("the sms cannot be read");
            continue;
        };

        println!("gsm number found: {}", msisdn);
    }

    Ok(())
}
