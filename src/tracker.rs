use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const TRACKED_CARDS_PATH: &str = "data/tracked_cards.json";
pub const BUNDLED_TRACKED_CARDS_JSON: &str = include_str!("../data/tracked_cards.json");
pub const DEFAULT_TRACKER_COLUMNS: usize = 5;
pub const DEFAULT_TRACKER_ROWS: usize = 3;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TrackedCardsFile {
    #[serde(default)]
    pub layout: TrackedLayoutSpec,
    #[serde(default)]
    pub cards: Vec<TrackedCardSpec>,
    #[serde(default)]
    pub groups: Vec<TrackedGroupSpec>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct TrackedLayoutSpec {
    #[serde(default)]
    pub columns: Option<usize>,
    #[serde(default)]
    pub rows: Option<usize>,
}

impl TrackedLayoutSpec {
    pub fn columns(&self) -> usize {
        self.columns.unwrap_or(DEFAULT_TRACKER_COLUMNS).max(1)
    }

    pub fn rows(&self) -> usize {
        self.rows.unwrap_or(DEFAULT_TRACKER_ROWS).max(1)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TrackedCardSpec {
    pub id: u16,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub target: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TrackedGroupSpec {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub image: Option<String>,
}

pub fn tracked_cards_file_from_json(
    json_data: &str,
) -> Result<TrackedCardsFile, serde_json::Error> {
    serde_json::from_str::<TrackedCardsFile>(json_data)
}

pub fn tracked_card_specs_from_json(
    json_data: &str,
) -> Result<Vec<TrackedCardSpec>, serde_json::Error> {
    tracked_cards_file_from_json(json_data).map(|tracked_cards| tracked_cards.cards)
}

#[derive(Debug)]
pub enum TrackedCardsLoadError {
    Read { path: PathBuf, source: io::Error },
    Parse { source: serde_json::Error },
}

impl fmt::Display for TrackedCardsLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "could not read {}: {source}", path.display())
            }
            Self::Parse { source } => {
                write!(formatter, "could not parse tracked cards JSON: {source}")
            }
        }
    }
}

impl std::error::Error for TrackedCardsLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source } => Some(source),
        }
    }
}

pub fn tracked_card_specs_from_file_or_bundled(
    path: impl AsRef<Path>,
) -> Result<Vec<TrackedCardSpec>, TrackedCardsLoadError> {
    tracked_cards_file_from_file_or_bundled(path).map(|tracked_cards| tracked_cards.cards)
}

pub fn tracked_cards_file_from_file_or_bundled(
    path: impl AsRef<Path>,
) -> Result<TrackedCardsFile, TrackedCardsLoadError> {
    let path = path.as_ref();
    let json_data = match fs::read_to_string(path) {
        Ok(json_data) => json_data,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            BUNDLED_TRACKED_CARDS_JSON.to_owned()
        }
        Err(error) => {
            return Err(TrackedCardsLoadError::Read {
                path: path.to_path_buf(),
                source: error,
            });
        }
    };

    tracked_cards_file_from_json(&json_data)
        .map_err(|source| TrackedCardsLoadError::Parse { source })
}

pub fn bundled_tracked_card_specs() -> Result<Vec<TrackedCardSpec>, serde_json::Error> {
    tracked_card_specs_from_json(BUNDLED_TRACKED_CARDS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

    #[test]
    fn parses_tracked_groups_from_json() {
        let tracked_cards = tracked_cards_file_from_json(
            r#"{
                "cards": [
                    { "id": 1, "target": 3 }
                ],
                "groups": [
                    {
                        "id": "thunders",
                        "name": "Thunders",
                        "image": "assets/groups/thunders.webp"
                    }
                ]
            }"#,
        )
        .expect("tracked cards file should parse");

        assert_eq!(tracked_cards.cards.len(), 1);
        assert_eq!(tracked_cards.groups.len(), 1);
        assert_eq!(tracked_cards.groups[0].id, "thunders");
        assert_eq!(tracked_cards.groups[0].name, "Thunders");
        assert_eq!(
            tracked_cards.groups[0].image.as_deref(),
            Some("assets/groups/thunders.webp")
        );
    }

    #[test]
    fn parses_tracker_layout_from_json() {
        let tracked_cards = tracked_cards_file_from_json(
            r#"{
                "layout": {
                    "columns": 4,
                    "rows": 2
                },
                "cards": [
                    { "id": 1, "target": 3 }
                ]
            }"#,
        )
        .expect("tracked cards file should parse");

        assert_eq!(tracked_cards.layout.columns(), 4);
        assert_eq!(tracked_cards.layout.rows(), 2);
    }

    #[test]
    fn defaults_and_clamps_tracker_layout_dimensions() {
        let tracked_cards =
            tracked_cards_file_from_json(r#"{ "layout": { "columns": 0, "rows": 0 } }"#)
                .expect("tracked cards file should parse");

        assert_eq!(tracked_cards.layout.columns(), 1);
        assert_eq!(tracked_cards.layout.rows(), 1);

        let tracked_cards = tracked_cards_file_from_json(r#"{ "cards": [{ "id": 1 }] }"#)
            .expect("tracked cards file should parse");

        assert_eq!(tracked_cards.layout.columns(), DEFAULT_TRACKER_COLUMNS);
        assert_eq!(tracked_cards.layout.rows(), DEFAULT_TRACKER_ROWS);
    }

    #[test]
    fn loads_tracked_card_specs_from_runtime_file() {
        let path = std::env::temp_dir().join(format!(
            "ygofm-motto-tracked-cards-{}.json",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"{
                "cards": [
                    { "id": 44, "label": "Runtime config", "target": 2 }
                ]
            }"#,
        )
        .expect("temporary tracked cards file should be writable");

        let specs = tracked_card_specs_from_file_or_bundled(&path)
            .expect("runtime tracked cards file should parse");

        fs::remove_file(path).expect("temporary tracked cards file should be removed");

        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].id, 44);
        assert_eq!(specs[0].label.as_deref(), Some("Runtime config"));
        assert_eq!(specs[0].target, Some(2));
    }

    #[test]
    fn falls_back_to_bundled_tracked_cards_when_runtime_file_is_missing() {
        let path = std::env::temp_dir().join(format!(
            "ygofm-motto-missing-tracked-cards-{}.json",
            std::process::id()
        ));

        let specs = tracked_card_specs_from_file_or_bundled(path)
            .expect("missing runtime file should use bundled tracked cards");

        assert!(!specs.is_empty());
    }
}
