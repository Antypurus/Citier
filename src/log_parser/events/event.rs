use std::str::FromStr;

use crate::log_parser::event_type::EventType;
use crate::log_parser::events::combat_log_info::CombatLogInfoEvent;
use crate::log_parser::events::combatant_info::CombatantInfoEvent;
use crate::log_parser::events::zone_change::{MapChangeEvent, ZoneChangeEvent};

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

        eprintln!("Unparsed Event:{:?}", event_type);

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
    MapChange(MapChangeEvent),
    CombatantInfo(CombatantInfoEvent),
}

impl Event {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> Event {
        let event_type = EventType::from_str(fields[0]).unwrap_or(EventType::Unknown);
        return match event_type {
            EventType::LogVersion => Event::CombatLogInfo(CombatLogInfoEvent::from_fields(timestamp, fields)),
            EventType::ZoneChange => Event::ZoneChange(ZoneChangeEvent::from_fields(timestamp, fields)),
            EventType::MapChange => Event::MapChange(MapChangeEvent::from_fields(timestamp, fields)),
            EventType::CombatantInfo => Event::CombatantInfo(CombatantInfoEvent::from_fields(timestamp, fields)),
            _ => Event::NormalEvent(EventBase::from_fields(timestamp, fields, event_type)),
        };
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        return match self {
            Event::NormalEvent(event) => event.to_string(),
            Event::CombatLogInfo(event) => event.to_string(),
            Event::ZoneChange(event) => event.to_string(),
            Event::MapChange(event) => event.to_string(),
            _ => String::default(),
        };
    }
}
