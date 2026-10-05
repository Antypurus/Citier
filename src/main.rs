use core::iter::Iterator;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;
use std::str::FromStr;

const COMTBAT_LOG_PATH: &str = "WoWCombatLog-090726_205649.txt";

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";

// raw combat event strings
const COMBAT_LOG_EVENT_TYPE_LOG_VERSION: &str = "COMBAT_LOG_VERSION";
// damage event raw strings
const COMBAT_LOG_EVENT_TYPE_SPELL_DAMAGE: &str = "SPELL_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_DAMAGE: &str = "SPELL_PERIODIC_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_SPELL_BUILDING_DAMAGE: &str = "SPELL_BUILDING_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_RANGE_DAMAGE: &str = "RANGE_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_SWING_DAMAGE: &str = "SWING_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_SWING_DAMAGE_LANDED: &str = "SWING_DAMAGE_LANDED";
const COMBAT_LOG_EVENT_TYPE_ENVIRONMENTAL_DAMAGE: &str = "ENVIRONMENTAL_DAMAGE";
const COMBAT_LOG_EVENT_TYPE_SPELL_ABSORBED: &str = "SPELL_ABSORBED";
const COMBAT_LOG_EVENT_TYPE_SPELL_HEAL_ABSORBED: &str = "SPELL_HEAL_ABSORBED";
const COMBAT_LOG_EVENT_TYPE_DAMAGE_SPLIT: &str = "DAMAGE_SPLIT";
const COMBAT_LOG_EVENT_TYPE_SPELL_INSTAKILL: &str = "SPELL_INSTAKILL";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventType {
    LogVersion,
    // Damage Events
    SpellDamage,
    SpellPeriodicDamage,
    SpellBuildingDamager,
    RangeDamage,
    SwingDamage,
    SwingDamageLanded,
    EnvironmentalDamage,
    SpellAbsorbed,
    SpellHealAbsorbed,
    DamageSplit,
    SpellInstakill,
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            COMBAT_LOG_EVENT_TYPE_LOG_VERSION => Ok(EventType::LogVersion),
            //Damage Events
            COMBAT_LOG_EVENT_TYPE_SPELL_DAMAGE => Ok(EventType::SpellDamage),
            COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_DAMAGE => Ok(EventType::SpellPeriodicDamage),
            COMBAT_LOG_EVENT_TYPE_SPELL_BUILDING_DAMAGE => Ok(EventType::SpellBuildingDamager),
            COMBAT_LOG_EVENT_TYPE_RANGE_DAMAGE => Ok(EventType::RangeDamage),
            COMBAT_LOG_EVENT_TYPE_SWING_DAMAGE => Ok(EventType::SwingDamage),
            COMBAT_LOG_EVENT_TYPE_SWING_DAMAGE_LANDED => Ok(EventType::SwingDamageLanded),
            COMBAT_LOG_EVENT_TYPE_ENVIRONMENTAL_DAMAGE => Ok(EventType::EnvironmentalDamage),
            COMBAT_LOG_EVENT_TYPE_SPELL_ABSORBED => Ok(EventType::SpellAbsorbed),
            COMBAT_LOG_EVENT_TYPE_SPELL_HEAL_ABSORBED => Ok(EventType::SpellHealAbsorbed),
            COMBAT_LOG_EVENT_TYPE_DAMAGE_SPLIT => Ok(EventType::DamageSplit),
            COMBAT_LOG_EVENT_TYPE_SPELL_INSTAKILL => Ok(EventType::SpellInstakill),
            // unknown event
            other => Err(format!("Unknown Event Type: {other}")),
        }
    }
}

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
                                eprintln!("{error}");
                                continue;
                            }
                        };
                        println!("{timestamp} - {event_type:?} - {:?}", &split_fields[1..]);
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

    return ExitCode::SUCCESS;
}
