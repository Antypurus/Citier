use std::fs;

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
        for field in event.split(',') {
            println!("Field: {field}")
        }
    }
}
