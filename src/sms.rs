use tokio_serial::{self, SerialPortBuilderExt};

use crate::at_command::{self, cmd};

pub fn prepare_sms_content(message: Option<String>) -> Vec<u8> {
    let message = message.unwrap_or("EHLO".into());
    let mut message_bytes = message.as_bytes().to_vec();
    message_bytes.push(0x1A); // ctrl+z

    message_bytes
}

pub async fn send_sms(port: &str, receiver: &str, message: Option<String>) -> Option<()> {
    let mut serial = tokio_serial::new(port, 115_200).open_native_async().ok()?;

    let _ = at_command::send_cmd_and_wait(cmd::AT, &mut serial).await;
    let _ = at_command::send_cmd_and_wait(cmd::SWITCH_TXT_SMS_MODE, &mut serial).await;
    let _ = at_command::send_cmd_and_wait(cmd::SET_GSM_CHARSET, &mut serial).await;
    let _ = at_command::send_cmd_and_wait(&cmd::start_sms(&receiver), &mut serial).await;
    let _ = at_command::send_cmd_and_wait(&prepare_sms_content(message), &mut serial).await;

    at_command::wait_cmgs(&mut serial).await
}
