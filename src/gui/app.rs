use std::error::Error;

use eframe::egui;
use ygofm_motto::{
    Card, CardDatabase, TRACKED_CARDS_PATH, TrackedCardSpec,
    tracked_card_specs_from_file_or_bundled,
};

use super::images::CardImageCache;
use super::widgets::draw_card_control;

const DEFAULT_CARD_TARGET: u32 = 3;

pub fn run_card_tracker() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;
    let tracker = CardTrackerApp::from_database(database);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([860.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "YGOFM Card Tracker",
        options,
        Box::new(|_creation_context| Ok(Box::new(tracker))),
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
pub(super) struct TrackedCard {
    pub(super) spec: TrackedCardSpec,
    pub(super) card: Card,
    pub(super) count: u32,
}

struct CardTrackerApp {
    tracked_cards: Vec<TrackedCard>,
    card_images: CardImageCache,
    missing_card_ids: Vec<u16>,
    load_error: Option<String>,
}

impl CardTrackerApp {
    fn from_database(database: CardDatabase) -> Self {
        let mut missing_card_ids = Vec::new();

        let specs = match tracked_card_specs_from_file_or_bundled(TRACKED_CARDS_PATH) {
            Ok(specs) => specs,
            Err(error) => {
                return Self {
                    tracked_cards: Vec::new(),
                    card_images: CardImageCache::new(),
                    missing_card_ids,
                    load_error: Some(error.to_string()),
                };
            }
        };

        let tracked_cards = specs
            .into_iter()
            .filter_map(|spec| {
                let card_id = spec.id;
                match database.card(card_id).cloned() {
                    Some(card) => Some(TrackedCard {
                        spec,
                        card,
                        count: 0,
                    }),
                    None => {
                        missing_card_ids.push(card_id);
                        None
                    }
                }
            })
            .collect();

        Self {
            tracked_cards,
            card_images: CardImageCache::new(),
            missing_card_ids,
            load_error: None,
        }
    }
}

impl eframe::App for CardTrackerApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            if let Some(load_error) = &self.load_error {
                ui.colored_label(
                    egui::Color32::from_rgb(190, 46, 46),
                    format!("Could not load tracked cards JSON: {load_error}"),
                );
                return;
            }

            if !self.missing_card_ids.is_empty() {
                let missing_ids = self
                    .missing_card_ids
                    .iter()
                    .map(|id| format!("#{id:03}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                ui.colored_label(
                    egui::Color32::from_rgb(190, 119, 0),
                    format!("Ignored unknown tracked card ids: {missing_ids}"),
                );
                ui.add_space(8.0);
            }

            if self.tracked_cards.is_empty() {
                ui.label("No tracked cards were configured in data/tracked_cards.json.");
                return;
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(14.0, 14.0);

                    for tracked_card in &mut self.tracked_cards {
                        let title = tracked_card.title();
                        let texture = self.card_images.texture_for(context, tracked_card.card.id);
                        draw_card_control(ui, texture, tracked_card, title);
                    }
                });
            });
        });
    }
}

impl TrackedCard {
    pub(super) fn target(&self) -> u32 {
        self.spec.target.unwrap_or(DEFAULT_CARD_TARGET)
    }

    fn title(&self) -> String {
        self.spec
            .label
            .clone()
            .unwrap_or_else(|| format!("#{:03} {}", self.card.id, self.card.name))
    }

    pub(super) fn count_label(&self) -> String {
        self.count.to_string()
    }
}
