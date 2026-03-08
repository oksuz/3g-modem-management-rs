pub mod at {
    use std::io::Error;

    use std::time::Duration;
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::time::sleep;
    use tokio::time::timeout;
    use tokio_serial::SerialStream;

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

    pub async fn send_cmd_and_wait(cmd: &[u8], serial: &mut SerialStream) -> Result<(), Error> {
        let _ = serial.write_all(cmd).await?;
        sleep(Duration::from_millis(150)).await;
        Ok(())
    }

    pub async fn wait_cmgs(serial: &mut SerialStream) -> Option<()> {
        let mut current_try = 1;
        loop {
            if current_try > 5 {
                break;
            }

            let mut buff = [0u8; 1024];
            let read_result = timeout(Duration::from_secs(2), serial.read(&mut buff)).await;

            if let Ok(Ok(read_bytes)) = read_result {
                let response: String = String::from_utf8_lossy(&buff[..read_bytes]).into();

                let has_cmgs = response
                    .lines()
                    .map(str::trim)
                    .find_map(|l| l.strip_prefix("+CMGS:"))
                    .map(str::trim);

                match has_cmgs {
                    Some(_) => {
                        return Some(());
                    }
                    None => {
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            current_try += 1;
        }

        None
    }
}
