use std::fmt::Write;

use crate::card::{Card, DropRank, Duelist};
use crate::database::CardDatabase;
use crate::labels::{attribute_name, card_type_name, guardian_star_name};

impl CardDatabase {
    pub fn format_card_details(&self, card: &Card) -> String {
        let mut output = String::new();

        self.write_card_identity(&mut output, card);
        self.write_equip_targets(&mut output, card.id);
        self.write_fusions(&mut output, card.id);
        self.write_rituals(&mut output, card.id);
        self.write_card_drops(&mut output, card.id);
        self.write_opponent_decks(&mut output, card.id);

        output
    }

    pub fn format_duelist_details(&self, duelist: &Duelist) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "Duelist #{:02} - {}", duelist.id, duelist.name);
        let _ = writeln!(output, "Hand size: {}", duelist.hand_size);

        self.write_duelist_deck_pool(&mut output, duelist.id);
        self.write_duelist_drop_pools(&mut output, duelist.id);

        output
    }

    pub fn format_duelist_deck(&self, duelist: &Duelist) -> String {
        self.format_duelist_details(duelist)
    }

    fn write_card_identity(&self, output: &mut String, card: &Card) {
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
    }

    fn write_equip_targets(&self, output: &mut String, card_id: u16) {
        let equip_targets = self.equip_targets_for(card_id);
        if equip_targets.is_empty() {
            let _ = writeln!(output, "Equip targets: none");
            return;
        }

        let _ = writeln!(output, "Equip targets ({}):", equip_targets.len());
        for target in equip_targets {
            let _ = writeln!(output, "  - {}", self.describe_card_ref(target));
        }
    }

    fn write_fusions(&self, output: &mut String, card_id: u16) {
        let fusions = self.fusions_for(card_id);
        if fusions.is_empty() {
            let _ = writeln!(output, "Fusions: none");
            return;
        }

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

    fn write_rituals(&self, output: &mut String, card_id: u16) {
        let rituals = self.rituals_for(card_id);
        if rituals.is_empty() {
            let _ = writeln!(output, "Rituals: none");
            return;
        }

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

    fn write_card_drops(&self, output: &mut String, card_id: u16) {
        let drops = self.drops_for_card(card_id);
        if drops.is_empty() {
            let _ = writeln!(output, "Drops: none");
            return;
        }

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

    fn write_opponent_decks(&self, output: &mut String, card_id: u16) {
        let opponent_decks = self.opponent_decks_for_card(card_id);
        if opponent_decks.is_empty() {
            let _ = writeln!(output, "Opponent decks: none");
            return;
        }

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

    fn write_duelist_deck_pool(&self, output: &mut String, duelist_id: u8) {
        let deck = self.duelist_deck(duelist_id);

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
    }

    fn write_duelist_drop_pools(&self, output: &mut String, duelist_id: u8) {
        let drops = self.duelist_drops(duelist_id);
        let _ = writeln!(output, "Drop pools ({} cards):", drops.len());

        for rank in DropRank::ALL {
            let rank_entries = self.duelist_drops_for_rank(duelist_id, rank);
            if rank_entries.is_empty() {
                continue;
            }

            let _ = writeln!(output, "{} ({} cards):", rank.label(), rank_entries.len());
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
