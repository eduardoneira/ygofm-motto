use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Card {
    pub name: String,
    pub id: u16,
    pub guardian_star_a: u8,
    pub guardian_star_b: u8,
    #[serde(rename = "type")]
    pub card_type: u8,
    pub attack: u16,
    pub defense: u16,
    pub starchip_cost: u32,
    pub card_code: String,
    pub attribute: u8,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Fusion {
    pub card1: u16,
    pub card2: u16,
    pub result: u16,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Equip {
    pub equip_card: u16,
    pub target_card: u16,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Ritual {
    pub ritual_card: u16,
    pub card1: u16,
    pub card2: u16,
    pub card3: u16,
    pub result: u16,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Duelist {
    pub id: u8,
    pub name: String,
    pub hand_size: u8,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DuelistDeckEntry {
    pub duelist_id: u8,
    pub card_id: u16,
    pub weight: u16,
}

impl DuelistDeckEntry {
    pub const WEIGHT_DENOMINATOR: u16 = 2048;

    pub fn odds_percent(&self) -> f64 {
        f64::from(self.weight) / f64::from(Self::WEIGHT_DENOMINATOR) * 100.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DuelistDropEntry {
    pub duelist_id: u8,
    pub rank: String,
    pub card_id: u16,
    pub weight: u16,
}

impl DuelistDropEntry {
    pub const WEIGHT_DENOMINATOR: u16 = 2048;

    pub fn odds_percent(&self) -> f64 {
        f64::from(self.weight) / f64::from(Self::WEIGHT_DENOMINATOR) * 100.0
    }

    pub fn rank_label(&self) -> &'static str {
        rank_label(&self.rank)
    }

    pub fn rank_sort_key(&self) -> u8 {
        rank_sort_key(&self.rank)
    }
}

pub fn rank_label(rank: &str) -> &'static str {
    match rank {
        "SAPow" => "S/A POW",
        "BCD" => "B/C/D",
        "SATec" => "S/A TEC",
        _ => "Unknown",
    }
}

pub fn rank_sort_key(rank: &str) -> u8 {
    match rank {
        "SAPow" => 0,
        "BCD" => 1,
        "SATec" => 2,
        _ => u8::MAX,
    }
}
