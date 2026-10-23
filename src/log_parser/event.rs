use std::str::FromStr;
use std::{fmt, time};

use super::event_type::EventType;
use super::parse_utils::parse_bool;
use super::tokenizer::owning_event_tokenizer;

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

struct PullAura {
    source_guid: String,
    spell_id: u64,
    stack_data: String,
}

impl PullAura {
    pub fn create(source: String, spell_id: u64, stack_data: String) -> PullAura {
        return PullAura { source_guid: source, spell_id: spell_id, stack_data: stack_data };
    }
}

impl ToString for PullAura {
    fn to_string(&self) -> String {
        return format!("{} - {}", self.source_guid, self.spell_id);
    }
}

impl fmt::Debug for PullAura {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

pub struct CombatantInfoEvent {
    timestamp: String,
    player_guid: String,
    strength: u64,
    agility: u64,
    stamina: u64,
    intellect: u64,
    dodge: u64,
    parry: u64,
    block: u64,
    crit: u64,
    speed: u64,
    leech: u64,
    haste: u64,
    avoidance: u64,
    mastery: u64,
    versatility: u64,
    armor: u64,
    spec_id: u64,
    pull_auras: Vec<PullAura>,
}

impl CombatantInfoEvent {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> CombatantInfoEvent {
        let on_pull_auras = owning_event_tokenizer(fields[29].trim_start_matches('[').trim_end_matches(']'));
        let pull_auras: Vec<PullAura> =
            on_pull_auras.chunks_exact(3).map(|c| PullAura { source_guid: c[0].to_owned(), spell_id: c[1].parse().unwrap_or_default(), stack_data: c[2].to_owned() }).collect();

        return CombatantInfoEvent {
            timestamp: String::from_str(timestamp).unwrap_or_default(),
            player_guid: String::from_str(fields[1]).unwrap_or_default(),
            strength: fields[3].parse().unwrap_or_default(),
            agility: fields[4].parse().unwrap_or_default(),
            stamina: fields[5].parse().unwrap_or_default(),
            intellect: fields[6].parse().unwrap_or_default(),
            dodge: fields[8].parse().unwrap_or_default(),
            parry: fields[9].parse().unwrap_or_default(),
            block: fields[10].parse().unwrap_or_default(),
            crit: fields[11].parse().unwrap_or_default(),
            speed: fields[14].parse().unwrap_or_default(),
            leech: fields[15].parse().unwrap_or_default(),
            haste: fields[16].parse().unwrap_or_default(),
            avoidance: fields[19].parse().unwrap_or_default(),
            mastery: fields[20].parse().unwrap_or_default(),
            versatility: fields[21].parse().unwrap_or_default(),
            armor: fields[24].parse().unwrap_or_default(),
            spec_id: fields[25].parse().unwrap_or_default(),
            pull_auras: pull_auras,
        };
    }
}

impl ToString for CombatantInfoEvent {
    fn to_string(&self) -> String {
        let spec = match self.spec_id {
            //DK Specs
            250 => "Blood DK",
            251 => "Frost DK",
            252 => "Unholy DK",
            //DH Specs
            577 => "Havoc DH",
            581 => "Vengeance DH",
            1480 => "Devourer DH",
            //Druid  Specs
            102 => "Balance Druid",
            103 => "Feral Druid",
            104 => "Guardian Druid",
            105 => "Restoration Druid",
            //Evoker Specs
            1467 => "Devastation Evoker",
            1468 => "Preservation Evoker",
            1473 => "Augmentation Evoker",
            //Hunter Specs
            253 => "Beast Mastery Hunter",
            254 => "Marksmanship Hunter",
            255 => "Survival Hunter",
            //Mage Specs
            62 => "Arcane Mage",
            63 => "Fire Mage",
            64 => "Frost Mage",
            //Monk Specs
            268 => "Brewmaster Monk",
            270 => "Mistweaver Monk",
            269 => "Windwalkerr Monk",
            //Paladin Specs
            65 => "Holy Paladin",
            66 => "Protection Paladin",
            70 => "Retribution Paladin",
            //Priest Specs
            256 => "Discipline Priest",
            257 => "Holy Priest",
            258 => "Shadow Priest",
            //Rogue Specs
            259 => "Assassination Rogue",
            260 => "Outlaw Rogue",
            261 => "Subtlety Rogue",
            //Shaman Specs
            262 => "Elemental Shaman",
            263 => "Enhancement Shaman",
            264 => "Restoration Shaman",
            //Warlock Specs
            265 => "Affliction Warlock",
            266 => "Demonology Warlock",
            267 => "Destruction Warlock",
            //Warrior Specs
            71 => "Arms Warrior",
            72 => "Fury Warrior",
            73 => "Protection Warrior",
            //==================
            _ => "Unknown",
        };
        return format!(
            "{} - {:?}: {} Spec: {}
            \t Stamina: {}
            \t Main Stat: {}|{}|{}
            \t Crit: {}
            \t Haste: {}
            \t Mastery: {}
            \t Versatility: {}
            \t Pull Auras: {:?}\n",
            self.timestamp,
            EventType::CombatantInfo,
            self.player_guid,
            spec,
            self.stamina,
            self.intellect,
            self.agility,
            self.strength,
            self.crit,
            self.haste,
            self.mastery,
            self.versatility,
            self.pull_auras
        );
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
