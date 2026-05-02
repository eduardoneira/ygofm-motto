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
