use std::str::FromStr;

use crate::log_parser::event_type::EventType;
use crate::log_parser::events::combat_event::EventBase;
use crate::log_parser::events::combat_log_info::CombatLogInfoEvent;
use crate::log_parser::events::combatant_info::CombatantInfoEvent;
use crate::log_parser::events::unhandled_event::UnhandledEvent;
use crate::log_parser::events::zone_change::{MapChangeEvent, ZoneChangeEvent};

pub enum Event {
    NormalEvent(EventBase),
    CombatLogInfo(CombatLogInfoEvent),
    ZoneChange(ZoneChangeEvent),
    MapChange(MapChangeEvent),
    CombatantInfo(CombatantInfoEvent),
    Unhandled(UnhandledEvent),
}

impl Event {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> Event {
        let event_type = EventType::from_str(fields[0]).unwrap_or(EventType::Unknown);
        return match event_type {
            EventType::LogVersion => Event::CombatLogInfo(CombatLogInfoEvent::from_fields(timestamp, fields)),
            EventType::ZoneChange => Event::ZoneChange(ZoneChangeEvent::from_fields(timestamp, fields)),
            EventType::MapChange => Event::MapChange(MapChangeEvent::from_fields(timestamp, fields)),
            EventType::CombatantInfo => Event::CombatantInfo(CombatantInfoEvent::from_fields(timestamp, fields)),
            _ => Event::Unhandled(UnhandledEvent { event_type: event_type }),
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
