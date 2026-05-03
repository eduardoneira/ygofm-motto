use serde::Deserialize;

pub const TRACKED_CARDS_JSON: &str = include_str!("../data/tracked_cards.json");

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TrackedCardsFile {
    pub cards: Vec<TrackedCardSpec>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TrackedCardSpec {
    pub id: u16,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub target: Option<u32>,
}

pub fn tracked_card_specs_from_json(
    json_data: &str,
) -> Result<Vec<TrackedCardSpec>, serde_json::Error> {
    serde_json::from_str::<TrackedCardsFile>(json_data).map(|tracked_cards| tracked_cards.cards)
}

pub fn bundled_tracked_card_specs() -> Result<Vec<TrackedCardSpec>, serde_json::Error> {
    tracked_card_specs_from_json(TRACKED_CARDS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tracked_card_specs_from_json() {
        let specs = tracked_card_specs_from_json(
            r#"{
                "cards": [
                    { "id": 1, "target": 3 },
                    { "id": 35, "label": "Starter", "target": 1 }
                ]
            }"#,
        )
        .expect("tracked card specs should parse");

        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0].id, 1);
        assert_eq!(specs[0].target, Some(3));
        assert_eq!(specs[1].label.as_deref(), Some("Starter"));
    }
}
