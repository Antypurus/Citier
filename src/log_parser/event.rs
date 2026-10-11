use std::{str::FromStr, time};

use super::event_type::EventType;

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
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> EventBase {
        return EventBase {
            timestamp: timestamp.parse().unwrap_or(String::default()),
            event_type: EventType::from_str(fields[0]).unwrap_or(EventType::Unknown),
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
}

impl ToString for EventBase {
    fn to_string(&self) -> String {
        return format!("{} - {:?}: {} -> {}", self.timestamp, self.event_type, self.source_name, self.target_name);
    }
}
