use std::fs;

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";
const COMBAT_LOG_FIELD_DELIMITER: char = ',';

fn main() {
    let read_result = fs::read_to_string("WoWCombatLog-090726_205649.txt");
    let log = match read_result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to read logfile: {err}");
            return;
        }
    };

    for event in log.lines() {
        let Some((timestamp, combat_event)) = event.split_once(COMBAT_LOG_TIMESTAMP_DELIMITER)
        else {
            eprintln!("Malformed Combat Log Line Found");
            continue;
        };

        let fields: Vec<&str> = combat_event.split(COMBAT_LOG_FIELD_DELIMITER).collect();
        println!("{} - {}", timestamp, fields[0])
    }
}
