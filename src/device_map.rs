use dashmap::DashMap;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ModemInfo {
    pub imei: String,
    pub port: String,
    pub last_seen: Instant,
}

pub type DeviceMap = DashMap<String, ModemInfo>;
