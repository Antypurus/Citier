use core::iter::Iterator;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use std::process::exit;
use std::str::FromStr;

const COMTBAT_LOG_PATH: &str = "WoWCombatLog-090726_205649.txt";

const COMBAT_LOG_TIMESTAMP_DELIMITER: &str = "  ";

// raw combat event strings
// Encounter & Metadata Events
const COMBAT_LOG_EVENT_TYPE_LOG_VERSION: &str = "COMBAT_LOG_VERSION";
const COMBAT_LOG_EVENT_TYPE_ENCOUNTER_START: &str = "ENCOUNTER_START";
const COMBAT_LOG_EVENT_TYPE_ENCOUNTER_END: &str = "ENCOUNTER_END";
const COMBAT_LOG_EVENT_TYPE_CHALLENGE_MODE_START: &str = "CHALLENGE_MODE_START";
const COMBAT_LOG_EVENT_TYPE_CHALLENGE_MODE_END: &str = "CHALLENGE_MODE_END";
const COMBAT_LOG_EVENT_TYPE_ZONE_CHANGE: &str = "ZONE_CHANGE";
const COMBAT_LOG_EVENT_TYPE_MAP_CHANGE: &str = "MAP_CHANGE";
const COMBAT_LOG_EVENT_TYPE_UNIT_HEALTH: &str = "UNIT_HEALTH";
const COMBAT_LOG_EVENT_TYPE_WORLD_MARKER_PLACED: &str = "WORLD_MARKER_PLACED";
const COMBAT_LOG_EVENT_TYPE_WORLD_MARKER_REMOVED: &str = "WORLD_MARKER_REMOVED";
const COMBAT_LOG_EVENT_TYPE_COMBATANT_INFO: &str = "COMBATANT_INFO";
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
// special events
const COMBAT_LOG_EVENT_TYPE_SPELL_INTERRUPT: &str = "SPELL_INTERRUPT"; //NOTE: cast kick
const COMBAT_LOG_EVENT_TYPE_SPELL_DISPEL: &str = "SPELL_DISPEL"; //NOTE: Dispel
const COMBAT_LOG_EVENT_TYPE_SPELL_STOLEN: &str = "SPELL_STOLEN"; //NOTE: Spellsteal 
const COMBAT_LOG_EVENT_TYPE_SPELL_SUMMON: &str = "SPELL_SUMMON"; //NOTE: Summon (used for pet attribution)
const COMBAT_LOG_EVENT_TYPE_SPELL_RESURRECT: &str = "SPELL_RESURRECT"; //NOTE: Combat Res
const COMBAT_LOG_EVENT_TYPE_SWING_MISSED: &str = "SWING_MISSED"; //NOTE: Swing Whiffed
const COMBAT_LOG_EVENT_TYPE_SPELL_MISSED: &str = "SPELL_MISSED"; //NOTE: Spell Whiffed
const COMBAT_LOG_EVENT_TYPE_RANGE_MISSED: &str = "RANGE_MISSED"; //NOTE: Range Whiffed
const COMBAT_LOG_EVENT_TYPE_UNIT_DIED: &str = "UNIT_DIED"; //NOTE: some unit died (there might an associated killing blow event)
const COMBAT_LOG_EVENT_TYPE_UNIT_DESTROYED: &str = "UNIT_DESTROYED"; //NOTE: pet/totem/object detroyed
const COMBAT_LOG_EVENT_TYPE_UNIT_DISSIPATES: &str = "UNIT_DISSIPATES"; //NOTE: unit fades out (similar to unit killed)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventType {
    //Encounter & Metadata Events
    LogVersion,
    EncounterStart,
    EncounterEnd,
    ChallengeModeStart,
    ChallengeModeEnd,
    ZoneChange,
    MapChange,
    UnitHealth,
    WorldMarkerPlaced,
    WorldMarketRemoved,
    CombatantInfo,
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
    // Special Events
    SpellInterrupt,
    SpellDispel,
    SpellStolen,
    SpellSummon,
    SpellResurrect,
    SwingMissed,
    SpellMissed,
    RangeMissed,
    UnitDied,
    UnitDestroyed,
    UnitDissipates,
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            //Encounter & Metadata Events
            COMBAT_LOG_EVENT_TYPE_LOG_VERSION => Ok(EventType::LogVersion),
            COMBAT_LOG_EVENT_TYPE_ENCOUNTER_START => Ok(EventType::EncounterStart),
            COMBAT_LOG_EVENT_TYPE_ENCOUNTER_END => Ok(EventType::EncounterEnd),
            COMBAT_LOG_EVENT_TYPE_CHALLENGE_MODE_START => Ok(EventType::ChallengeModeStart),
            COMBAT_LOG_EVENT_TYPE_CHALLENGE_MODE_END => Ok(EventType::ChallengeModeEnd),
            COMBAT_LOG_EVENT_TYPE_ZONE_CHANGE => Ok(EventType::ZoneChange),
            COMBAT_LOG_EVENT_TYPE_MAP_CHANGE => Ok(EventType::MapChange),
            COMBAT_LOG_EVENT_TYPE_UNIT_HEALTH => Ok(EventType::UnitHealth),
            COMBAT_LOG_EVENT_TYPE_WORLD_MARKER_PLACED => Ok(EventType::WorldMarkerPlaced),
            COMBAT_LOG_EVENT_TYPE_WORLD_MARKER_REMOVED => Ok(EventType::WorldMarketRemoved),
            COMBAT_LOG_EVENT_TYPE_COMBATANT_INFO => Ok(EventType::CombatantInfo),
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
            // Special Events
            COMBAT_LOG_EVENT_TYPE_SPELL_INTERRUPT => Ok(EventType::SpellInterrupt),
            COMBAT_LOG_EVENT_TYPE_SPELL_DISPEL => Ok(EventType::SpellDispel),
            COMBAT_LOG_EVENT_TYPE_SPELL_STOLEN => Ok(EventType::SpellStolen),
            COMBAT_LOG_EVENT_TYPE_SPELL_SUMMON => Ok(EventType::SpellSummon),
            COMBAT_LOG_EVENT_TYPE_SPELL_RESURRECT => Ok(EventType::SpellResurrect),
            COMBAT_LOG_EVENT_TYPE_SWING_MISSED => Ok(EventType::SwingMissed),
            COMBAT_LOG_EVENT_TYPE_SPELL_MISSED => Ok(EventType::SpellMissed),
            COMBAT_LOG_EVENT_TYPE_RANGE_MISSED => Ok(EventType::RangeMissed),
            COMBAT_LOG_EVENT_TYPE_UNIT_DIED => Ok(EventType::UnitDied),
            COMBAT_LOG_EVENT_TYPE_UNIT_DESTROYED => Ok(EventType::UnitDestroyed),
            COMBAT_LOG_EVENT_TYPE_UNIT_DISSIPATES => Ok(EventType::UnitDissipates),
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

    println!("");
    println!("Unhandled Event Count: {}", unhandled_event_types.len());
    for event in unhandled_event_types {
        println!("\t -> {event}");
    }
    return ExitCode::SUCCESS;
}
