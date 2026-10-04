use core::iter::Iterator;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";
const _COMBAT_LOG_FIELD_DELIMITER: char = ',';
const COMTBAT_LOG_PATH: &str = "WoWCombatLog-090726_205649.txt";

fn _event_tokenizer(event: &str) -> Vec<&str> {
    let mut fields: Vec<&str> = Vec::new();

    let mut slice_start: usize = 0;
    let mut scope_stack: Vec<u8> = Vec::new();

    for (i, character) in event.bytes().enumerate() {
        match (scope_stack.last(), character) {
            (Some(b'"'), b'"') => {
                scope_stack.pop();
            }
            (Some(b'['), b']') => {
                scope_stack.pop();
            }
            (_, b'[') => {
                scope_stack.push(b'[');
            }
            (_, b'"') => {
                scope_stack.push(b'"');
            }
            (None, b',') => {
                fields.push(&event[slice_start..i]);
                slice_start = i + 1;
            }
            (_, _) => {}
        }
    }

    return fields;
}

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
                match event.split_once(COMBAT_LOG_TIMESTAMP_DELIMITER) {
                    Some((timestamp, fields)) => {
                        let split_fields: Vec<&str> = _event_tokenizer(&fields);
                        println!("{timestamp} - {:?}", split_fields);
                    }
                    None => {
                        eprintln!("Failed to split log line");
                    }
                };
                event.clear();
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    }

    return ExitCode::SUCCESS;
}
