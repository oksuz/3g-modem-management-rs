use tokio_serial::{self, SerialPortBuilderExt};

use crate::at_command::at;

pub fn prepare_sms_content(message: Option<&str>) -> Vec<u8> {
    let message = message.unwrap_or("EHLO");
    let mut message_bytes = message.as_bytes().to_vec();
    message_bytes.push(0x1A); // ctrl+z

    message_bytes
}

pub async fn send_sms(port: &str, receiver: &str, message: Option<&str>) -> Option<()> {
    let mut serial = tokio_serial::new(port, 115_200).open_native_async().ok()?;

    let _ = at::send_cmd_and_wait(at::AT, &mut serial).await;
    let _ = at::send_cmd_and_wait(at::SWITCH_TXT_SMS_MODE, &mut serial).await;
    let _ = at::send_cmd_and_wait(at::SET_GSM_CHARSET, &mut serial).await;
    let _ = at::send_cmd_and_wait(&at::start_sms(&receiver), &mut serial).await;
    let _ = at::send_cmd_and_wait(&prepare_sms_content(message), &mut serial).await;

    at::wait_cmgs(&mut serial).await
}
