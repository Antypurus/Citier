use std::str::FromStr;

use crate::log_parser::event_type::EventType;
use crate::log_parser::parse_utils::parse_bool;

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
