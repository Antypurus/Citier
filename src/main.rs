mod log_parser {
    pub mod event_type;
}

use log_parser::event_type::EventType;

use core::iter::Iterator;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;
use std::str::FromStr;

// Makeup of a WoW Combat Event:
// 11 Base Parameters:
//  0 - Timestamp
//  1 - Event Type String ("SPELL_DAMAGE")
//  2 - bool indicating weather the source unit is hidden (hidden = empty source string)
//  3 - GUID String for Source Unit
//  4 - Source Unit Name
//  5 - Flags for the source Unit
//  6 - Source Unit Raid Flags
//  7 - GUID String for destination unit
//  8 - Destination Unit Name
//  9 - Destination Unit Flags
//  10 - Destination Unit Raid Flags
// 0 - 3 Prefix Related Params
// 17 Advanced Parameters
// 0 - 10 Suffix Parameters

const COMTBAT_LOG_PATH: &str = "Archive-WoWCombatLog-083126_184807.txt";

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";

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

    let mut unhandled_event_types: HashSet<String> = HashSet::new();
    let mut event = String::new();
    loop {
        event.clear();
        match file_reader.read_line(&mut event) {
            Ok(0) => {
                //nothing left to read we can break
                break;
            }
            Ok(_) => {
                match event.split_once(COMBAT_LOG_TIMESTAMP_DELIMITER) {
                    Some((timestamp, fields)) => {
                        let split_fields: Vec<&str> = _event_tokenizer(&fields);
                        let event_type = match EventType::from_str(split_fields[0]) {
                            Ok(event) => event,
                            Err(error) => {
                                unhandled_event_types.insert(String::from_str(&split_fields[0]).unwrap());
                                eprintln!("{error}");
                                continue;
                            }
                        };
                        println!("{timestamp} - {event_type:?} - {:?}({})", &split_fields[1..], split_fields[1..].len());
                    }
                    None => {
                        eprintln!("Failed to split log line");
                    }
                };
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    }

    println!("");
    println!("Unhandled Event Count: {}", unhandled_event_types.len());
    for event in unhandled_event_types {
        println!("\t -> {event}");
    }
    return ExitCode::SUCCESS;
}
