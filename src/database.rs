use std::fmt::Write;

use serde::de::DeserializeOwned;

use crate::card::{Card, Duelist, DuelistDeckEntry, DuelistDropEntry, Equip, Fusion, Ritual};
use crate::labels::{attribute_name, card_type_name, guardian_star_name};

pub const CARDS_CSV: &str = include_str!("../data/cards.csv");
pub const FUSIONS_CSV: &str = include_str!("../data/fusions.csv");
pub const EQUIPS_CSV: &str = include_str!("../data/equips.csv");
pub const RITUALS_CSV: &str = include_str!("../data/rituals.csv");
pub const DUELISTS_CSV: &str = include_str!("../data/duelists.csv");
pub const DUELIST_DECKS_CSV: &str = include_str!("../data/duelist_decks.csv");
pub const DUELIST_DROPS_CSV: &str = include_str!("../data/duelist_drops.csv");

#[derive(Debug, Clone)]
pub struct CardDatabase {
    cards: Vec<Card>,
    fusions: Vec<Fusion>,
    equips: Vec<Equip>,
    rituals: Vec<Ritual>,
    duelists: Vec<Duelist>,
    duelist_decks: Vec<DuelistDeckEntry>,
    duelist_drops: Vec<DuelistDropEntry>,
}

impl CardDatabase {
    pub fn from_csv_tables(
        cards_csv: &str,
        fusions_csv: &str,
        equips_csv: &str,
        rituals_csv: &str,
        duelists_csv: &str,
        duelist_decks_csv: &str,
        duelist_drops_csv: &str,
    ) -> Result<Self, csv::Error> {
        Ok(Self {
            cards: parse_csv(cards_csv)?,
            fusions: parse_csv(fusions_csv)?,
            equips: parse_csv(equips_csv)?,
            rituals: parse_csv(rituals_csv)?,
            duelists: parse_csv(duelists_csv)?,
            duelist_decks: parse_csv(duelist_decks_csv)?,
            duelist_drops: parse_csv(duelist_drops_csv)?,
        })
    }

    pub fn from_bundled_csv() -> Result<Self, csv::Error> {
        Self::from_csv_tables(
            CARDS_CSV,
            FUSIONS_CSV,
            EQUIPS_CSV,
            RITUALS_CSV,
            DUELISTS_CSV,
            DUELIST_DECKS_CSV,
            DUELIST_DROPS_CSV,
        )
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

    pub fn duelists(&self) -> &[Duelist] {
        &self.duelists
    }

    pub fn duelist(&self, id: u8) -> Option<&Duelist> {
        self.duelists.iter().find(|duelist| duelist.id == id)
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

    pub fn duelist_deck(&self, duelist_id: u8) -> Vec<&DuelistDeckEntry> {
        let mut deck_entries = self
            .duelist_decks
            .iter()
            .filter(|entry| entry.duelist_id == duelist_id)
            .collect::<Vec<_>>();

        deck_entries.sort_by(|left, right| {
            right
                .weight
                .cmp(&left.weight)
                .then_with(|| left.card_id.cmp(&right.card_id))
        });

        deck_entries
    }

    pub fn opponent_decks_for_card(&self, card_id: u16) -> Vec<&DuelistDeckEntry> {
        let mut deck_entries = self
            .duelist_decks
            .iter()
            .filter(|entry| entry.card_id == card_id)
            .collect::<Vec<_>>();

        deck_entries.sort_by(|left, right| {
            right
                .weight
                .cmp(&left.weight)
                .then_with(|| left.duelist_id.cmp(&right.duelist_id))
        });

        deck_entries
    }

    pub fn duelist_drops(&self, duelist_id: u8) -> Vec<&DuelistDropEntry> {
        let mut drop_entries = self
            .duelist_drops
            .iter()
            .filter(|entry| entry.duelist_id == duelist_id)
            .collect::<Vec<_>>();

        drop_entries.sort_by(|left, right| {
            left.rank_sort_key()
                .cmp(&right.rank_sort_key())
                .then_with(|| right.weight.cmp(&left.weight))
                .then_with(|| left.card_id.cmp(&right.card_id))
        });

        drop_entries
    }

    pub fn duelist_drops_for_rank(&self, duelist_id: u8, rank: &str) -> Vec<&DuelistDropEntry> {
        let mut drop_entries = self
            .duelist_drops
            .iter()
            .filter(|entry| entry.duelist_id == duelist_id && entry.rank == rank)
            .collect::<Vec<_>>();

        drop_entries.sort_by(|left, right| {
            right
                .weight
                .cmp(&left.weight)
                .then_with(|| left.card_id.cmp(&right.card_id))
        });

        drop_entries
    }

    pub fn drops_for_card(&self, card_id: u16) -> Vec<&DuelistDropEntry> {
        let mut drop_entries = self
            .duelist_drops
            .iter()
            .filter(|entry| entry.card_id == card_id)
            .collect::<Vec<_>>();

        drop_entries.sort_by(|left, right| {
            right
                .weight
                .cmp(&left.weight)
                .then_with(|| left.duelist_id.cmp(&right.duelist_id))
                .then_with(|| left.rank_sort_key().cmp(&right.rank_sort_key()))
        });

        drop_entries
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

        let drops = self.drops_for_card(card.id);
        if drops.is_empty() {
            let _ = writeln!(output, "Drops: none");
        } else {
            let _ = writeln!(output, "Drops ({}):", drops.len());
            for entry in drops {
                let _ = writeln!(
                    output,
                    "  - {} {}: {:.2}% (weight {})",
                    self.describe_duelist_ref(entry.duelist_id),
                    entry.rank_label(),
                    entry.odds_percent(),
                    entry.weight
                );
            }
        }

        let opponent_decks = self.opponent_decks_for_card(card.id);
        if opponent_decks.is_empty() {
            let _ = writeln!(output, "Opponent decks: none");
        } else {
            let _ = writeln!(output, "Opponent decks ({}):", opponent_decks.len());
            for entry in opponent_decks {
                let _ = writeln!(
                    output,
                    "  - {}: {:.2}% (weight {})",
                    self.describe_duelist_ref(entry.duelist_id),
                    entry.odds_percent(),
                    entry.weight
                );
            }
        }

        output
    }

    pub fn format_duelist_details(&self, duelist: &Duelist) -> String {
        let mut output = String::new();
        let deck = self.duelist_deck(duelist.id);
        let drops = self.duelist_drops(duelist.id);

        let _ = writeln!(output, "Duelist #{:02} - {}", duelist.id, duelist.name);
        let _ = writeln!(output, "Hand size: {}", duelist.hand_size);
        let _ = writeln!(output, "Deck pool ({} cards):", deck.len());

        for entry in deck {
            let _ = writeln!(
                output,
                "  - {}: {:.2}% (weight {})",
                self.describe_card_ref(entry.card_id),
                entry.odds_percent(),
                entry.weight
            );
        }

        let _ = writeln!(output, "Drop pools ({} cards):", drops.len());
        for rank in ["SAPow", "BCD", "SATec"] {
            let rank_entries = self.duelist_drops_for_rank(duelist.id, rank);
            if rank_entries.is_empty() {
                continue;
            }

            let _ = writeln!(
                output,
                "{} ({} cards):",
                rank_entries[0].rank_label(),
                rank_entries.len()
            );
            for entry in rank_entries {
                let _ = writeln!(
                    output,
                    "  - {}: {:.2}% (weight {})",
                    self.describe_card_ref(entry.card_id),
                    entry.odds_percent(),
                    entry.weight
                );
            }
        }

        output
    }

    pub fn format_duelist_deck(&self, duelist: &Duelist) -> String {
        self.format_duelist_details(duelist)
    }

    fn describe_card_ref(&self, id: u16) -> String {
        self.card(id)
            .map(|card| format!("#{:03} {}", card.id, card.name))
            .unwrap_or_else(|| format!("#{id:03} <unknown>"))
    }

    fn describe_duelist_ref(&self, id: u8) -> String {
        self.duelist(id)
            .map(|duelist| format!("#{:02} {}", duelist.id, duelist.name))
            .unwrap_or_else(|| format!("#{id:02} <unknown>"))
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
        assert_eq!(database.duelists.len(), 39);
        assert_eq!(database.duelist_decks.len(), 3_681);
        assert_eq!(database.duelist_drops.len(), 8_666);

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
    fn loads_duelists_and_weighted_deck_pools() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        let simon = database.duelist(1).expect("duelist 1 should exist");
        assert_eq!(simon.name, "Simon Muran");
        assert_eq!(simon.hand_size, 5);

        let simon_deck = database.duelist_deck(1);
        assert_eq!(simon_deck.len(), 32);
        assert_eq!(
            simon_deck.iter().map(|entry| entry.weight).sum::<u16>(),
            DuelistDeckEntry::WEIGHT_DENOMINATOR
        );
        assert!(
            simon_deck
                .iter()
                .any(|entry| entry.card_id == 343 && entry.weight == 100)
        );

        let shadow_specter_decks = database.opponent_decks_for_card(9);
        assert!(
            shadow_specter_decks
                .iter()
                .any(|entry| entry.duelist_id == 1)
        );
    }

    #[test]
    fn loads_weighted_drop_pools() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        let simon_drops = database.duelist_drops(1);
        assert_eq!(simon_drops.len(), 139);

        let simon_pow_drops = database.duelist_drops_for_rank(1, "SAPow");
        assert_eq!(simon_pow_drops.len(), 47);
        assert_eq!(
            simon_pow_drops
                .iter()
                .map(|entry| entry.weight)
                .sum::<u16>(),
            DuelistDropEntry::WEIGHT_DENOMINATOR
        );
        assert!(
            simon_pow_drops
                .iter()
                .any(|entry| entry.card_id == 9 && entry.weight == 90)
        );

        let shadow_specter_drops = database.drops_for_card(9);
        assert!(
            shadow_specter_drops
                .iter()
                .any(|entry| entry.duelist_id == 1 && entry.rank == "SAPow")
        );
    }

    #[test]
    fn every_duelist_pool_totals_the_game_weight_denominator() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");

        for duelist in database.duelists() {
            let deck = database.duelist_deck(duelist.id);
            assert!(
                !deck.is_empty(),
                "{} should have deck entries",
                duelist.name
            );
            assert_eq!(
                deck.iter().map(|entry| entry.weight).sum::<u16>(),
                DuelistDeckEntry::WEIGHT_DENOMINATOR,
                "{} deck weights should total 2048",
                duelist.name
            );

            for entry in deck {
                assert!(
                    database.card(entry.card_id).is_some(),
                    "{} deck references unknown card #{}",
                    duelist.name,
                    entry.card_id
                );
            }

            for rank in ["SAPow", "BCD", "SATec"] {
                let drops = database.duelist_drops_for_rank(duelist.id, rank);
                assert!(
                    !drops.is_empty(),
                    "{} should have {rank} drop entries",
                    duelist.name
                );
                assert_eq!(
                    drops.iter().map(|entry| entry.weight).sum::<u16>(),
                    DuelistDropEntry::WEIGHT_DENOMINATOR,
                    "{} {rank} drop weights should total 2048",
                    duelist.name
                );

                for entry in drops {
                    assert!(
                        database.card(entry.card_id).is_some(),
                        "{} {rank} drops reference unknown card #{}",
                        duelist.name,
                        entry.card_id
                    );
                }
            }
        }
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
        assert!(details.contains("Drops ("));
        assert!(details.contains("Opponent decks"));
    }

    #[test]
    fn formats_duelist_details_with_percentages() {
        let database = CardDatabase::from_bundled_csv().expect("card CSV tables should parse");
        let duelist = database.duelist(1).expect("duelist 1 should exist");
        let details = database.format_duelist_details(duelist);

        assert!(details.contains("Duelist #01 - Simon Muran"));
        assert!(details.contains("Deck pool (32 cards):"));
        assert!(details.contains("#343 Sparks: 4.88% (weight 100)"));
        assert!(details.contains("Drop pools (139 cards):"));
        assert!(details.contains("S/A POW (47 cards):"));
        assert!(details.contains("#009 Shadow Specter: 4.39% (weight 90)"));
    }
}
