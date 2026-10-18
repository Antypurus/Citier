use std::str::FromStr;

use super::event_type::EventType;
use super::parse_utils::parse_bool;

pub struct CombatLogInfoEvent {
    timestamp: String,
    log_version: u32,
    advanced_logging_enabled: bool,
    build_version: String,
    project_id: u32,
}

impl CombatLogInfoEvent {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> CombatLogInfoEvent {
        return CombatLogInfoEvent {
            timestamp: String::from_str(timestamp).unwrap_or(String::default()),
            log_version: fields[1].parse().unwrap_or(0),
            advanced_logging_enabled: parse_bool(fields[3]).unwrap_or_default(),
            build_version: String::from_str(fields[5]).unwrap_or(String::default()),
            project_id: fields[7].parse().unwrap_or(0),
        };
    }
}

impl ToString for CombatLogInfoEvent {
    fn to_string(&self) -> String {
        return format!(
            "{} - {:?}: Spec Version: {}, Advanced Logging: {}, WoW Build: {}",
            &self.timestamp,
            EventType::LogVersion,
            self.log_version,
            self.advanced_logging_enabled,
            &self.build_version
        );
    }
}

pub struct ZoneChangeEvent {
    timestamp: String,
    instance_id: u64,
    zone_name: String,
    difficulty: u64,
}

impl ZoneChangeEvent {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> ZoneChangeEvent {
        return ZoneChangeEvent {
            timestamp: String::from_str(timestamp).unwrap_or_default(),
            instance_id: fields[1].parse().unwrap_or_default(),
            zone_name: String::from_str(fields[2]).unwrap_or_default(),
            difficulty: fields[3].parse().unwrap_or_default(),
        };
    }
}

impl ToString for ZoneChangeEvent {
    fn to_string(&self) -> String {
        let difficulty = match self.difficulty {
            1 => "Normal Dungeon",
            2 => "Heroic Dungeon",
            7 => "LFR",
            14 => "Normal Raid",
            15 => "Heroic Raid",
            16 => "Mythic Raid",
            17 => "LFR",
            23 => "Mythic Dungeon",
            _ => "Unknown",
        };
        return format!("{} -> {:?}: Zone: {} ({}) Difficulty: {}", &self.timestamp, EventType::ZoneChange, &self.zone_name, self.instance_id, difficulty);
    }
}

pub struct EventBase {
    pub timestamp: String,
    pub event_type: EventType,
    // source fields
    pub source_guid: String,
    pub source_name: String,
    pub source_flags: u32,
    pub source_raid_flags: u32,
    // target fields
    pub target_guid: String,
    pub target_name: String,
    pub target_flags: u32,
    pub target_raid_flags: u32,
}

impl EventBase {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>, event_type: EventType) -> EventBase {
        if fields.len() >= 9 {
            return EventBase {
                timestamp: timestamp.parse().unwrap_or(String::default()),
                event_type: event_type,
                source_guid: fields[1].parse().unwrap_or(String::default()),
                source_name: fields[2].parse().unwrap_or(String::default()),
                source_flags: fields[3].parse().unwrap_or(0),
                source_raid_flags: fields[4].parse().unwrap_or(0),
                target_guid: fields[5].parse().unwrap_or(String::default()),
                target_name: fields[6].parse().unwrap_or(String::default()),
                target_flags: fields[7].parse().unwrap_or(0),
                target_raid_flags: fields[8].parse().unwrap_or(0),
            };
        }

        return EventBase {
            timestamp: String::default(),
            event_type,
            source_guid: String::default(),
            source_name: String::default(),
            source_flags: 0,
            source_raid_flags: 0,
            target_guid: String::default(),
            target_name: String::default(),
            target_flags: 0,
            target_raid_flags: 0,
        };
    }
}

impl ToString for EventBase {
    fn to_string(&self) -> String {
        return format!("{} - {:?}: {} -> {}", self.timestamp, self.event_type, self.source_name, self.target_name);
    }
}

pub enum Event {
    NormalEvent(EventBase),
    CombatLogInfo(CombatLogInfoEvent),
    ZoneChange(ZoneChangeEvent),
}

impl Event {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> Event {
        let event_type = EventType::from_str(fields[0]).unwrap_or(EventType::Unknown);
        match event_type {
            EventType::LogVersion => {
                return Event::CombatLogInfo(CombatLogInfoEvent::from_fields(timestamp, fields));
            }
            EventType::ZoneChange => {
                return Event::ZoneChange(ZoneChangeEvent::from_fields(timestamp, fields));
            }
            _ => {
                return Event::NormalEvent(EventBase::from_fields(timestamp, fields, event_type));
            }
        }
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::NormalEvent(event) => {
                return event.to_string();
            }
            Event::CombatLogInfo(event) => {
                return event.to_string();
            }
            Event::ZoneChange(event) => {
                return event.to_string();
            }
            _ => {
                return String::default();
            }
        };
    }
}
