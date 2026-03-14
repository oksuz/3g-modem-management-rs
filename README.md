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

## Usage

Configure the environment variables in `.env` file and run the application to automatically detect and register SIM cards.
