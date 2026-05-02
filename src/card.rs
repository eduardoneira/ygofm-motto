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

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum DropRank {
    #[serde(rename = "SAPow")]
    SAPow,
    #[serde(rename = "BCD")]
    Bcd,
    #[serde(rename = "SATec")]
    SATec,
}

impl DropRank {
    pub const ALL: [Self; 3] = [Self::SAPow, Self::Bcd, Self::SATec];

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "SAPow" => Some(Self::SAPow),
            "BCD" => Some(Self::Bcd),
            "SATec" => Some(Self::SATec),
            _ => None,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::SAPow => "SAPow",
            Self::Bcd => "BCD",
            Self::SATec => "SATec",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SAPow => "S/A POW",
            Self::Bcd => "B/C/D",
            Self::SATec => "S/A TEC",
        }
    }

    pub fn sort_key(self) -> u8 {
        match self {
            Self::SAPow => 0,
            Self::Bcd => 1,
            Self::SATec => 2,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DuelistDropEntry {
    pub duelist_id: u8,
    pub rank: DropRank,
    pub card_id: u16,
    pub weight: u16,
}

impl DuelistDropEntry {
    pub const WEIGHT_DENOMINATOR: u16 = 2048;

    pub fn odds_percent(&self) -> f64 {
        f64::from(self.weight) / f64::from(Self::WEIGHT_DENOMINATOR) * 100.0
    }

    pub fn rank_label(&self) -> &'static str {
        self.rank.label()
    }

    pub fn rank_sort_key(&self) -> u8 {
        self.rank.sort_key()
    }
}

pub fn rank_label(rank: &str) -> &'static str {
    DropRank::from_code(rank).map_or("Unknown", DropRank::label)
}

pub fn rank_sort_key(rank: &str) -> u8 {
    DropRank::from_code(rank).map_or(u8::MAX, DropRank::sort_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_drop_rank_codes_to_domain_values() {
        assert_eq!(DropRank::from_code("SAPow"), Some(DropRank::SAPow));
        assert_eq!(DropRank::from_code("BCD"), Some(DropRank::Bcd));
        assert_eq!(DropRank::from_code("SATec"), Some(DropRank::SATec));
        assert_eq!(DropRank::from_code("???"), None);
    }

    #[test]
    fn keeps_drop_rank_labels_and_sort_order_explicit() {
        assert_eq!(DropRank::SAPow.code(), "SAPow");
        assert_eq!(DropRank::SAPow.label(), "S/A POW");
        assert_eq!(DropRank::Bcd.label(), "B/C/D");
        assert_eq!(DropRank::SATec.label(), "S/A TEC");

        assert_eq!(
            DropRank::ALL,
            [DropRank::SAPow, DropRank::Bcd, DropRank::SATec]
        );
        assert_eq!(rank_label("not-a-rank"), "Unknown");
        assert_eq!(rank_sort_key("not-a-rank"), u8::MAX);
    }
}
