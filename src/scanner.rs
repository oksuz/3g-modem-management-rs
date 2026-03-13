use std::time::Duration;

use glob::glob;
use tokio::io::AsyncReadExt;
use tokio::time::timeout;
use tokio_serial::{self, SerialPortBuilderExt};

use crate::at_command::{self, cmd};
use crate::parser::get_imei_from_ati;

pub fn scan_ports() -> Vec<String> {
    glob("/dev/ttyUSB*")
        .expect("unable to read glob pattern")
        .filter_map(Result::ok)
        .map(|p| p.display().to_string())
        .collect()
}

pub async fn probe_port(port: &str) -> Option<(&str, Option<String>)> {
    let mut serial = tokio_serial::new(port, 115_200).open_native_async().ok()?;

    let _ = at_command::send_cmd_and_wait(cmd::ATI, &mut serial).await;

    let mut buff = [0u8; 1024];
    let read = timeout(Duration::from_secs(2), serial.read(&mut buff)).await;

    match read {
        Ok(Ok(n)) => {
            if n == 0 {
                return None;
            }

            let imei = get_imei_from_ati(std::str::from_utf8(&buff[..n]).unwrap());
            return Some((port, imei));
        }
        Ok(Err(e)) => {
            eprintln!("error on reading from port({}): {:?}", port, e);
            None
        }
        Err(e) => {
            eprintln!("error on reading from port({}): {:?}", port, e);
            None
        }
    }
}
