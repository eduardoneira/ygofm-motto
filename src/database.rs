use std::fmt::Write;

use serde::de::DeserializeOwned;

use crate::card::{Card, Equip, Fusion, Ritual};
use crate::labels::{attribute_name, card_type_name, guardian_star_name};

pub const CARDS_CSV: &str = include_str!("../data/cards.csv");
pub const FUSIONS_CSV: &str = include_str!("../data/fusions.csv");
pub const EQUIPS_CSV: &str = include_str!("../data/equips.csv");
pub const RITUALS_CSV: &str = include_str!("../data/rituals.csv");

#[derive(Debug, Clone)]
pub struct CardDatabase {
    cards: Vec<Card>,
    fusions: Vec<Fusion>,
    equips: Vec<Equip>,
    rituals: Vec<Ritual>,
}

impl CardDatabase {
    pub fn from_csv_tables(
        cards_csv: &str,
        fusions_csv: &str,
        equips_csv: &str,
        rituals_csv: &str,
    ) -> Result<Self, csv::Error> {
        Ok(Self {
            cards: parse_csv(cards_csv)?,
            fusions: parse_csv(fusions_csv)?,
            equips: parse_csv(equips_csv)?,
            rituals: parse_csv(rituals_csv)?,
        })
    }

    pub fn from_bundled_csv() -> Result<Self, csv::Error> {
        Self::from_csv_tables(CARDS_CSV, FUSIONS_CSV, EQUIPS_CSV, RITUALS_CSV)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn card(&self, id: u16) -> Option<&Card> {
        self.cards.iter().find(|card| card.id == id)
    }

    pub fn fusions_for(&self, card_id: u16) -> Vec<&Fusion> {
        self.fusions
            .iter()
            .filter(|fusion| fusion.card1 == card_id)
            .collect()
    }

    pub fn equip_targets_for(&self, equip_card_id: u16) -> Vec<u16> {
        self.equips
            .iter()
            .filter(|equip| equip.equip_card == equip_card_id)
            .map(|equip| equip.target_card)
            .collect()
    }

    pub fn rituals_for(&self, ritual_card_id: u16) -> Vec<&Ritual> {
        self.rituals
            .iter()
            .filter(|ritual| ritual.ritual_card == ritual_card_id)
            .collect()
    }

    pub fn format_card_details(&self, card: &Card) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "Card #{:03} - {}", card.id, card.name);
        let _ = writeln!(
            output,
            "Type: {} ({})",
            card_type_name(card.card_type),
            card.card_type
        );
        let _ = writeln!(
            output,
            "Attribute: {} ({})",
            attribute_name(card.attribute),
            card.attribute
        );
        let _ = writeln!(
            output,
            "Guardian stars: {} ({}) / {} ({})",
            guardian_star_name(card.guardian_star_a),
            card.guardian_star_a,
            guardian_star_name(card.guardian_star_b),
            card.guardian_star_b
        );
        let _ = writeln!(output, "ATK / DEF: {} / {}", card.attack, card.defense);
        let _ = writeln!(output, "Password: {}", card.card_code);
        let _ = writeln!(output, "Starchip cost: {}", card.starchip_cost);

        let equip_targets = self.equip_targets_for(card.id);
        if equip_targets.is_empty() {
            let _ = writeln!(output, "Equip targets: none");
        } else {
            let _ = writeln!(output, "Equip targets ({}):", equip_targets.len());
            for target in equip_targets {
                let _ = writeln!(output, "  - {}", self.describe_card_ref(target));
            }
        };

        let fusions = self.fusions_for(card.id);
        if fusions.is_empty() {
            let _ = writeln!(output, "Fusions: none");
        } else {
            let _ = writeln!(output, "Fusions ({}):", fusions.len());
            for fusion in fusions {
                let _ = writeln!(
                    output,
                    "  - {} + {} => {}",
                    self.describe_card_ref(fusion.card1),
                    self.describe_card_ref(fusion.card2),
                    self.describe_card_ref(fusion.result)
                );
            }
        }

        let rituals = self.rituals_for(card.id);
        if rituals.is_empty() {
            let _ = writeln!(output, "Rituals: none");
        } else {
            let _ = writeln!(output, "Rituals ({}):", rituals.len());
            for ritual in rituals {
                let _ = writeln!(
                    output,
                    "  - {} + {} + {} with {} => {}",
                    self.describe_card_ref(ritual.card1),
                    self.describe_card_ref(ritual.card2),
                    self.describe_card_ref(ritual.card3),
                    self.describe_card_ref(ritual.ritual_card),
                    self.describe_card_ref(ritual.result)
                );
            }
        }

        output
    }

    fn describe_card_ref(&self, id: u16) -> String {
        self.card(id)
            .map(|card| format!("#{:03} {}", card.id, card.name))
            .unwrap_or_else(|| format!("#{id:03} <unknown>"))
    }
}

fn parse_csv<T>(csv_data: &str) -> Result<Vec<T>, csv::Error>
where
    T: DeserializeOwned,
{
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    reader.deserialize().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_all_cards_from_bundled_csv_tables() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        assert_eq!(database.len(), 722);
        assert_eq!(
            database.card(1).expect("card 1 should exist").name,
            "Blue-eyes White Dragon"
        );
        assert_eq!(
            database.card(722).expect("card 722 should exist").name,
            "Magician of Black Chaos"
        );
    }

    #[test]
    fn looks_up_cards_by_forbidden_memories_number() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");
        let dark_magician = database.card(35).expect("card 35 should exist");

        assert_eq!(dark_magician.name, "Dark Magician");
        assert_eq!(dark_magician.attack, 2500);
        assert_eq!(dark_magician.defense, 2100);
        assert_eq!(card_type_name(dark_magician.card_type), "Spellcaster");
    }

    #[test]
    fn returns_none_for_numbers_outside_the_card_list() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        assert!(database.card(0).is_none());
        assert!(database.card(723).is_none());
    }

    #[test]
    fn parses_normalized_fusions_equips_and_rituals() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        assert_eq!(database.fusions.len(), 25_131);
        assert_eq!(database.equips.len(), 4_041);
        assert_eq!(database.rituals.len(), 24);

        let mystical_elf_fusions = database.fusions_for(2);
        assert!(!mystical_elf_fusions.is_empty());
        assert!(
            mystical_elf_fusions
                .iter()
                .any(|fusion| fusion.card1 == 2 && fusion.card2 == 8 && fusion.result == 638)
        );

        let legendary_sword_targets = database.equip_targets_for(301);
        assert!(legendary_sword_targets.contains(&15));

        let rituals = database.rituals_for(665);
        assert_eq!(rituals[0].ritual_card, 665);
        assert_eq!(rituals[0].result, 362);
    }

    #[test]
    fn formats_card_details_with_resolved_names() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");
        let card = database.card(301).expect("card 301 should exist");
        let details = database.format_card_details(card);

        assert!(details.contains("Card #301 - Legendary Sword"));
        assert!(!details.contains("Description:"));
        assert!(!details.contains("Level:"));
        assert!(details.contains("Type: Equip (23)"));
        assert!(details.contains("Equip targets ("));
        assert!(details.contains("#015 Flame Swordsman"));
    }
}
