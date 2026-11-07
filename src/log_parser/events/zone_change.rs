use std::fmt;
use std::str::FromStr;

use crate::log_parser::event_type::EventType;

pub struct ZoneChangeEvent {
    timestamp: String,
    instance_id: u64,
    zone_name: String,
    difficulty: u64,
}

pub struct MapChangeEvent {
    timestamp: String,
    map_id: u64,
    map_name: String,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl MapChangeEvent {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> MapChangeEvent {
        assert!(fields.len() == 7);
        return MapChangeEvent {
            timestamp: String::from_str(timestamp).unwrap_or_default(),
            map_id: fields[1].parse().unwrap_or_default(),
            map_name: String::from_str(fields[2]).unwrap_or_default(),
            x_min: fields[3].parse().unwrap_or_default(),
            x_max: fields[4].parse().unwrap_or_default(),
            y_min: fields[5].parse().unwrap_or_default(),
            y_max: fields[6].parse().unwrap_or_default(),
        };
    }
}

impl ToString for MapChangeEvent {
    fn to_string(&self) -> String {
        return format!("{} - {:?}: {} ({}) Bounds: [{} , {}] - [{} , {}]", self.timestamp, EventType::ZoneChange, self.map_name, self.map_id, self.x_min, self.x_max, self.y_min, self.y_max);
    }
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
