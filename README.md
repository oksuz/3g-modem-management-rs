# MSISDN Extract

## Problem

SIM cards do not have MSISDN (phone numbers) written on them, making it difficult to identify which phone number belongs to which SIM card.

## Solution

This application extracts and registers MSISDN information for SIM cards using the following process:

1. Insert SIM card into a 3G modem
2. Extract the ICCID (SIM card serial number) from the SIM card using AT commands
3. Send an SMS from this SIM card with the ICCID as the message content
4. Query the SMS API to find the received SMS by matching the ICCID content
5. Extract the sender's MSISDN from the received SMS
6. Register the ICCID-MSISDN pair via the registration API

## Environment Variables

Create a `.env` file with the following variables:

- `ACTIVE_SIMS_API` - API endpoint to fetch active SIM devices
- `SMS_READ_API` - API endpoint to read received SMS messages
- `MSISDN_REGISTER_API` - API endpoint to register ICCID-MSISDN pairs
- `SMS_READ_MAX_RETRY` - Maximum number of retry attempts when checking for SMS (default: 15)
- `SMS_READ_SLEEP_DURATION` - Sleep duration in seconds between SMS check retries (default: 3)

## Requirements

This application works only on Linux systems with `/dev/ttyUSB*` device support for USB modems.

## Configuration

To run the application without sudo, configure your modem as follows:

### 1. Add your user to the dialout group

```bash
sudo usermod -aG dialout $USER
```

After running this command, log out and log back in for the changes to take effect.

### 2. Configure udev rules

First, identify your modem's vendor and product IDs using:

```bash
lsusb
```

Example output:

```
Bus 001 Device 009: ID 12d1:1436 Huawei Technologies Co., Ltd. Broadband stick
```

Create `/etc/udev/rules.d/99-usb-modem.rules` with the following content (adjust idVendor and idProduct according to your modem):

```
SUBSYSTEM=="tty", ATTRS{idVendor}=="12d1", ATTRS{idProduct}=="1436", MODE="0660", GROUP="dialout"
```

### 3. Stop ModemManager

ModemManager can interfere with modem communication. Disable it using:

```bash
sudo systemctl stop ModemManager
```

To disable it permanently:

```bash
sudo systemctl disable ModemManager
```

## Usage

Configure the environment variables in `.env` file and run the application to automatically detect and register SIM cards.
