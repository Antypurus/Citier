use std::fmt;
use std::str::FromStr;

use crate::log_parser::event_type::EventType;
use crate::log_parser::tokenizer::owning_event_tokenizer;

struct PullAura {
    source_guid: String,
    spell_id: u64,
    stack_data: String,
}

impl PullAura {
    pub fn create(source: String, spell_id: u64, stack_data: String) -> PullAura {
        return PullAura { source_guid: source, spell_id: spell_id, stack_data: stack_data };
    }

    pub fn from_field(auras_field: &str) -> Vec<PullAura> {
        let on_pull_auras = owning_event_tokenizer(auras_field.trim_start_matches('[').trim_end_matches(']'));
        let pull_auras: Vec<PullAura> =
            on_pull_auras.chunks_exact(3).map(|chunk| PullAura { source_guid: chunk[0].to_owned(), spell_id: chunk[1].parse().unwrap_or_default(), stack_data: chunk[2].to_owned() }).collect();
        return pull_auras;
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

pub struct PvETalentTreeEntry {
    node_id: u64,
    entry_id: u64,
    rank: u64,
}

impl PvETalentTreeEntry {
    pub fn from_field(talents_field: &str) -> Vec<PvETalentTreeEntry> {
        let talent_list = owning_event_tokenizer(talents_field.trim_start_matches("[").trim_end_matches("]"));

        let mut result: Vec<PvETalentTreeEntry> = Vec::new();
        for talent_entry in talent_list {
            let talent_info = owning_event_tokenizer(&talent_entry[1..talent_entry.len() - 1]);
            result.push(PvETalentTreeEntry {
                node_id: talent_info[0].parse().unwrap_or_default(),
                entry_id: talent_info[1].parse().unwrap_or_default(),
                rank: talent_info[2].parse().unwrap_or_default(),
            });
        }
        return result;
    }
}

impl fmt::Debug for PvETalentTreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Node:{} - Talent:{} Rank:{}", self.node_id, self.entry_id, self.rank);
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
    talents: Vec<PvETalentTreeEntry>,
    pvp_talents: String,
    items: String,
    pull_auras: Vec<PullAura>,
}

impl CombatantInfoEvent {
    pub fn from_fields(timestamp: &str, fields: &Vec<&str>) -> CombatantInfoEvent {
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
            talents: PvETalentTreeEntry::from_field(fields[26]),
            pvp_talents: String::from_str(fields[27]).unwrap_or_default(),
            items: String::from_str(fields[28]).unwrap_or_default(),
            pull_auras: PullAura::from_field(fields[29]),
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
            \t PvE Talents: {:?}
            \t PvP Talents: {}
            \t Equipped Items: {}
            \t Pull Auras: {:?}",
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
            self.talents,
            self.pvp_talents,
            self.items,
            self.pull_auras
        );
    }
}
