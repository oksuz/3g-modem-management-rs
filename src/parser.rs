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
