pub fn get_imei_from_ati(resp: &str) -> Option<String> {
    resp.lines()
        .map(str::trim)
        .find_map(|l| l.strip_prefix("IMEI:"))
        .map(str::trim)
        .map(str::to_string)
}

pub fn has_cmgs(resp: &str) -> Option<()> {
    let cmgs = resp
        .lines()
        .map(str::trim)
        .find_map(|l| l.strip_prefix("+CMGS:"))
        .map(str::trim);

    cmgs.map(|_| ())
}

pub fn get_dbm_from_rssi(resp: &str) -> Option<i32> {
    resp.lines()
        .map(str::trim)
        .find_map(|l| l.strip_prefix("+CSQ:"))
        .map(str::trim)
        .and_then(|v| v.split(',').next().map(|s| s.trim().to_string()))
        .and_then(|s| s.parse::<i32>().ok())
        .map(|rssi| -113 + (rssi * 2))
}
