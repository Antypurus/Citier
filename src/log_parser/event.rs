use std::str::FromStr;

use super::event_type::EventType;
use super::parse_utils::parse_bool;

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

pub enum Event {
    NormalEvent(EventBase),
    CombatLogInfo(CombatLogInfoEvent),
}

impl Event {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> Event {
        let event_type = EventType::from_str(fields[0]).unwrap_or(EventType::Unknown);
        match event_type {
            EventType::LogVersion => {
                return Event::CombatLogInfo(CombatLogInfoEvent::from_fields(timestamp, fields));
            }
            _ => {
                if fields.len() >= 9 {
                    return Event::NormalEvent(EventBase {
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
                    });
                }

                return Event::NormalEvent(EventBase {
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
                });
            }
        }
    }
}

impl ToString for EventBase {
    fn to_string(&self) -> String {
        return format!("{} - {:?}: {} -> {}", self.timestamp, self.event_type, self.source_name, self.target_name);
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::NormalEvent(event) => {
                return event.to_string();
            }
            Event::CombatLogInfo(event) => {
                return format!(
                    "{} - {:?}: Spec Version: {}, Advanced Logging: {}, WoW Build: {}",
                    &event.timestamp,
                    EventType::LogVersion,
                    event.log_version,
                    event.advanced_logging_enabled,
                    &event.build_version
                );
            }
            _ => {
                return String::default();
            }
        };
    }
}
