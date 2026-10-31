mod log_parser {
    pub mod event_type;
    pub mod parse_utils;
    pub mod tokenizer;

    pub mod events {
        pub mod event;
    }
}
use log_parser::event_type::EventType;
use log_parser::events::event::Event;
use log_parser::events::event::EventBase;
use log_parser::tokenizer::event_tokenizer;

use core::iter::Iterator;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;
use std::str::FromStr;

// Makeup of a WoW Combat Event:
// 10 Base Parameters:
//  0 - Timestamp
//  1 - Event Type String ("SPELL_DAMAGE")
//  2 - GUID String for Source Unit
//  3 - Source Unit Name
//  4 - Flags for the source Unit
//  5 - Source Unit Raid Flags
//  6 - GUID String for destination unit
//  7 - Destination Unit Name
//  8 - Destination Unit Flags
//  9 - Destination Unit Raid Flags
// 0 - 3 Prefix Related Params
// 17 Advanced Parameters
// 0 - 10 Suffix Parameters

const COMTBAT_LOG_PATH: &str = "Archive-WoWCombatLog-083126_184807.txt";

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";

fn main() -> std::process::ExitCode {
    let log_file = match fs::File::open(COMTBAT_LOG_PATH) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{error}");
            exit(-1);
        }
    };
    let mut file_reader = BufReader::new(log_file);

    let mut events: Vec<Event> = Vec::new();

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
                        let split_fields: Vec<&str> = event_tokenizer(&fields);
                        match EventType::from_str(split_fields[0]) {
                            Ok(event) => event,
                            Err(_error) => {
                                unhandled_event_types.insert(String::from_str(&split_fields[0]).unwrap());
                                //eprintln!("{error}");
                                continue;
                            }
                        };

                        events.push(Event::from_fields(&timestamp, &split_fields));
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

    for _event in &events {
        if let Event::CombatantInfo(info_event) = _event {
            println!("{}", info_event.to_string());
        }
    }

    println!("");
    println!("Unhandled Event Count: {}", unhandled_event_types.len());
    for event in unhandled_event_types {
        println!("\t -> {event}");
    }

    return ExitCode::SUCCESS;
}
