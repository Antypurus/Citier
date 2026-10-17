pub fn parse_bool(s: &str) -> Option<bool> {
    match s.trim() {
        "1" | "true" | "TRUE" | "True" => {
            return Some(true);
        }
        "0" | "false" | "FALSE" | "False" => {
            return Some(false);
        }
        _ => {
            return None;
        }
    }
}
