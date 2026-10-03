use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;

//const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";
//const COMBAT_LOG_FIELD_DELIMITER: char = ',';
const COMTBAT_LOG_PATH: &str = "WoWCombatLog-090726_205649.txt";

fn main() -> std::process::ExitCode {
    let log_file = match fs::File::open(COMTBAT_LOG_PATH) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{error}");
            exit(-1);
        }
    };
    let mut file_reader = BufReader::new(log_file);

    let mut event = String::new();
    loop {
        match file_reader.read_line(&mut event) {
            Ok(0) => {
                //nothing left to read we can break
                break;
            }
            Ok(_) => {
                println!("{event}");
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    }

    return ExitCode::SUCCESS;
}
