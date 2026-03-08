use std::time::Duration;

use glob::glob;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, timeout};
use tokio_serial::{self, SerialPortBuilderExt};

use crate::at_command::at;
use crate::parser::parse_ati_response;

pub async fn scan_ports() -> Vec<String> {
    tokio::task::spawn_blocking(|| {
        glob("/dev/ttyUSB*")
            .expect("unable to read glob pattern")
            .filter_map(Result::ok)
            .map(|p| p.display().to_string())
            .collect()
    })
    .await
    .unwrap_or_default()
}

pub async fn probe_port(port: &str) -> Option<(String, Option<String>)> {
    let mut serial = tokio_serial::new(port, 115_200).open_native_async().ok()?;

    let _ = serial.write_all(at::ATI).await;
    let _ = serial.flush().await;

    sleep(Duration::from_millis(100)).await;

    let mut buff = [0u8; 1024];
    let read = timeout(Duration::from_secs(2), serial.read(&mut buff)).await;

    match read {
        Ok(Ok(n)) => {
            if n == 0 {
                return None;
            }

            return Some((
                port.to_string(),
                parse_ati_response(String::from_utf8_lossy(&buff[..n]).into()),
            ));
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
