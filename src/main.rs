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
// healing & resource events
const COMBAT_LOG_EVENT_TYPE_SPELL_HEAL: &str = "SPELL_HEAL"; //NOTE: Direct Heal
const COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_HEAL: &str = "SPELL_PERIODIC_HEAL"; //NOTE: HoT Tick
const COMBAT_LOG_EVENT_TYPE_SPELL_ENERGIZE: &str = "SPELL_ENERGIZE"; //NOTE: Resource Gain
const COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_ENERGIZE: &str = "SPELL_PERIODIC_ENERGIZE"; //NOTE: Periodic resource gain
const COMBAT_LOG_EVENT_TYPE_SPELL_DRAIN: &str = "SPELL_DRAIN"; //NOTE: Resource drain
const COMBAT_LOG_EVENT_TYPE_SPELL_LEECH: &str = "SPELL_LEECH"; //NOTE: Leech effect
// aura events
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_APLIED: &str = "SPELL_AURA_APPLIED"; //NOTE: Aura Applied
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REMOVED: &str = "SPELL_AURA_REMOVED"; //NOTE: Aura dropped
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REFRESH: &str = "SPELL_AURA_REFRESH"; //NOTE: Aura refreshed
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_APPLIED_DOSE: &str = "SPELL_AURA_APPLIED_DOSE"; //NOTE: Aura Stack Applied
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REMOVED_DOSE: &str = "SPELL_AURA_REMOVED_DOSE"; //NOTE: Aura Stack Consumed
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_BROKEN: &str = "SPELL_AURA_BROKEN"; //NOTE: Aura Broken By Damage
const COMBAT_LOG_EVENT_TYPE_SPELL_AURA_BROKEN_SPELL: &str = "SPELL_AURA_BROKEN_SPELL"; //NOTE: Aura Broken By Dispell/Cleanse
// cast events
const COMBAT_LOG_EVENT_TYPE_SPELL_CAST_START: &str = "SPELL_CAST_START"; //non-instant cast started
const COMBAT_LOG_EVENT_TYPE_SPELL_CAST_SUCCESS: &str = "SPELL_CAST_SUCCESS"; //cast finished (ability fired, not a damage event)
const COMBAT_LOG_EVENT_TYPE_SPELL_CAST_FAILED: &str = "SPELL_CAST_FAILED"; //cast interrupted (i.e. movement, out of range, etc...)

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
    // Heal & Resource Events
    SpellHeal,
    SpellPeriodicHeal,
    SpellEnergize,
    SpellPeriodicEnergize,
    SpellDrain,
    SpellLeech,
    // Aura Events
    SpellAuraApplied,
    SpellAuraRemoved,
    SpellAuraRefresh,
    SpellAuraDoseApplied,
    SpellAuraDoseRemoved,
    SpellAuraBroken,
    SpellAuraBrokenSpell,
    // cast events
    SpellCastStart,
    SpellCastSuccess,
    SpellCastFailed,
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
            // Heal & Resource Events
            COMBAT_LOG_EVENT_TYPE_SPELL_HEAL => Ok(EventType::SpellHeal),
            COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_HEAL => Ok(EventType::SpellPeriodicHeal),
            COMBAT_LOG_EVENT_TYPE_SPELL_ENERGIZE => Ok(EventType::SpellEnergize),
            COMBAT_LOG_EVENT_TYPE_SPELL_PERIODIC_ENERGIZE => Ok(EventType::SpellPeriodicEnergize),
            COMBAT_LOG_EVENT_TYPE_SPELL_DRAIN => Ok(EventType::SpellDrain),
            COMBAT_LOG_EVENT_TYPE_SPELL_LEECH => Ok(EventType::SpellLeech),
            // Aura Events
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_APLIED => Ok(EventType::SpellAuraApplied),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REMOVED => Ok(EventType::SpellAuraRemoved),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REFRESH => Ok(EventType::SpellAuraRefresh),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_APPLIED_DOSE => Ok(EventType::SpellAuraDoseApplied),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_REMOVED_DOSE => Ok(EventType::SpellAuraDoseRemoved),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_BROKEN => Ok(EventType::SpellAuraBroken),
            COMBAT_LOG_EVENT_TYPE_SPELL_AURA_BROKEN_SPELL => Ok(EventType::SpellAuraBrokenSpell),
            // Cast Events
            COMBAT_LOG_EVENT_TYPE_SPELL_CAST_START => Ok(EventType::SpellCastStart),
            COMBAT_LOG_EVENT_TYPE_SPELL_CAST_SUCCESS => Ok(EventType::SpellCastSuccess),
            COMBAT_LOG_EVENT_TYPE_SPELL_CAST_FAILED => Ok(EventType::SpellCastFailed),
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
