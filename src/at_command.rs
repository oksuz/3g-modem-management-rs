use std::io::Error;

use crate::parser::has_cmgs;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio_serial::SerialPortBuilderExt;
use tokio_serial::SerialStream;
pub mod cmd {
    pub const ATI: &[u8] = b"ATI\r";

    pub const AT: &[u8] = b"AT\r";

    pub const ASK_ICCID: &[u8] = b"AT^ICCID?\r";
    pub const ASK_OPERATOR: &[u8] = b"AT+COPS?\r";
    pub const ASK_SIGNAL: &[u8] = b"AT+CSQ\r";
    pub const ASK_MODEL: &[u8] = b"AT+CGMM\r";

    pub const SET_GSM_CHARSET: &[u8] = b"AT+CSCS=\"GSM\"\r";

    pub const SWITCH_TXT_SMS_MODE: &[u8] = b"AT+CMGF=1\r";
    pub const SWITCH_MODEM_MEMORY: &[u8] = b"AT+CPMS=\"ME\",\"ME\",\"ME\"\r";
    pub const SWITCH_SIM_MEMORY: &[u8] = b"AT+CPMS=\"SM\",\"SM\",\"SM\"\r";

    pub const CLEAR_ALL_SMS: &[u8] = b"AT+CMGD=1,4\r";
    pub const GET_UNREAD_SMS: &[u8] = b"AT+CMGL=\"REC UNREAD\"\r";
    pub const GET_ALL_SMS: &[u8] = b"AT+CMGL=\"ALL\"\r";

    pub fn start_sms(receiver: &str) -> Vec<u8> {
        format!("AT+CMGS=\"{}\"\r", receiver).into_bytes()
    }
}

pub async fn send_cmd_and_wait(cmd: &[u8], serial: &mut SerialStream) -> Result<(), Error> {
    serial.write_all(cmd).await?;
    sleep(Duration::from_millis(150)).await;
    Ok(())
}

pub async fn wait_cmgs(serial: &mut SerialStream) -> Option<()> {
    for _ in 0..5 {
        let mut buff = [0u8; 1024];
        let read_result = timeout(Duration::from_secs(2), serial.read(&mut buff)).await;

        if let Ok(Ok(read_bytes)) = read_result {
            let response = str::from_utf8(&buff[..read_bytes]).unwrap_or_default();
            match has_cmgs(response) {
                Some(_) => {
                    return Some(());
                }
                None => {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    None
}

pub async fn get_iccid(port: &str) -> Option<String> {
    let mut serial = tokio_serial::new(port, 115_200).open_native_async().ok()?;
    let _ = send_cmd_and_wait(cmd::ASK_ICCID, &mut serial).await;

    let mut buff = [0u8; 1024];
    let read_bytes = serial
        .read(&mut buff)
        .await
        .map_err(|e| {
            eprintln!("cannot read device output: {:?}", e);
            e
        })
        .ok()?;

    if read_bytes == 0 {
        return None;
    }

    let iccid = str::from_utf8(&buff[..read_bytes]).ok()?.to_string();
    let Some(iccid) = iccid
        .lines()
        .find_map(|l| l.strip_prefix("^ICCID:"))
        .map(|s| s.trim().trim_matches('"'))
    else {
        return None;
    };

    let iccid_bytes = iccid.as_bytes();
    let mut iccid = String::new();
    for iccid_chunk in iccid_bytes.chunks(2) {
        if iccid_chunk.len() == 2 {
            iccid.push(iccid_chunk[1] as char);
            iccid.push(iccid_chunk[0] as char);
        } else {
            iccid.push(iccid_chunk[0] as char);
        }
    }

    if iccid.len() == 19 {
        iccid.push('F');
    }

    Some(iccid)
}
