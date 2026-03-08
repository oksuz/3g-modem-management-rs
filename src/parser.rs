pub fn parse_ati_response(resp: String) -> Option<String> {
    resp.lines()
        .map(str::trim)
        .find_map(|l| l.strip_prefix("IMEI:"))
        .map(str::trim)
        .map(str::to_string)
}
