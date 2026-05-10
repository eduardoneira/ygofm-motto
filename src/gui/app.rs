use std::error::Error;

use eframe::egui;
use ygofm_motto::{
    Card, CardDatabase, TRACKED_CARDS_PATH, TrackedCardSpec, TrackedGroupSpec,
    tracked_cards_file_from_file_or_bundled,
};

use super::images::{CardImageCache, GroupImageCache};
use super::layout::TrackerGridLayout;
use super::widgets::{card_art_uv, draw_tracker_control};

const DEFAULT_CARD_TARGET: u32 = 3;

pub fn run_card_tracker() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;
    let tracker = CardTrackerApp::from_database(database);
    let initial_window_size = tracker.initial_window_size();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(initial_window_size),
        ..Default::default()
    };

    eframe::run_native(
        "YGOFM Card Tracker",
        options,
        Box::new(move |_creation_context| Ok(Box::new(tracker))),
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
pub(super) struct TrackedCard {
    pub(super) spec: TrackedCardSpec,
    pub(super) card: Card,
    pub(super) count: u32,
}

#[derive(Debug, Clone)]
pub(super) struct TrackedGroup {
    pub(super) spec: TrackedGroupSpec,
    pub(super) count: u32,
}

struct CardTrackerApp {
    tracked_cards: Vec<TrackedCard>,
    tracked_groups: Vec<TrackedGroup>,
    tracker_layout: TrackerGridLayout,
    did_request_compact_window_size: bool,
    card_images: CardImageCache,
    group_images: GroupImageCache,
    missing_card_ids: Vec<u16>,
    load_error: Option<String>,
}

impl CardTrackerApp {
    fn from_database(database: CardDatabase) -> Self {
        let mut missing_card_ids = Vec::new();

        let tracked_cards_file = match tracked_cards_file_from_file_or_bundled(TRACKED_CARDS_PATH) {
            Ok(tracked_cards_file) => tracked_cards_file,
            Err(error) => {
                return Self {
                    tracked_cards: Vec::new(),
                    tracked_groups: Vec::new(),
                    tracker_layout: TrackerGridLayout::new(1, 1),
                    did_request_compact_window_size: true,
                    card_images: CardImageCache::new(),
                    group_images: GroupImageCache::new(),
                    missing_card_ids,
                    load_error: Some(error.to_string()),
                };
            }
        };

        let tracked_cards = tracked_cards_file
            .cards
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
        let tracked_groups = tracked_cards_file
            .groups
            .into_iter()
            .map(|spec| TrackedGroup { spec, count: 0 })
            .collect();

        Self {
            tracked_cards,
            tracked_groups,
            tracker_layout: TrackerGridLayout::new(
                tracked_cards_file.layout.columns(),
                tracked_cards_file.layout.rows(),
            ),
            did_request_compact_window_size: false,
            card_images: CardImageCache::new(),
            group_images: GroupImageCache::new(),
            missing_card_ids,
            load_error: None,
        }
    }

    fn initial_window_size(&self) -> egui::Vec2 {
        self.tracker_layout
            .initial_window_size(self.tracker_count())
    }

    fn tracker_count(&self) -> usize {
        self.tracked_cards.len() + self.tracked_groups.len()
    }

    fn request_compact_window_size_once(
        &mut self,
        context: &egui::Context,
        compact_window_size: egui::Vec2,
    ) {
        if self.did_request_compact_window_size {
            return;
        }

        self.did_request_compact_window_size = true;

        let target_size = context.input(|input| {
            input
                .viewport()
                .monitor_size
                .map_or(compact_window_size, |monitor_size| {
                    egui::vec2(
                        compact_window_size.x.min(monitor_size.x),
                        compact_window_size.y.min(monitor_size.y),
                    )
                })
        });

        let current_size =
            context.input(|input| input.viewport().inner_rect.map(|rect| rect.size()));

        if current_size.is_some_and(|current_size| sizes_are_close(current_size, target_size)) {
            return;
        }

        context.send_viewport_cmd(egui::ViewportCommand::InnerSize(target_size));
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

            if self.tracked_cards.is_empty() && self.tracked_groups.is_empty() {
                ui.label("No tracked cards or groups were configured in data/tracked_cards.json.");
                return;
            }

            let metrics = self
                .tracker_layout
                .metrics_for_available_size(ui.available_size(), self.tracker_count());
            self.request_compact_window_size_once(context, metrics.compact_window_size);

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("tracked_cards_grid")
                    .num_columns(metrics.columns)
                    .spacing(metrics.spacing)
                    .show(ui, |ui| {
                        let mut tracker_index = 0;
                        for tracked_card in &mut self.tracked_cards {
                            let title = tracked_card.title();
                            let target = tracked_card.target();
                            let fallback_label = format!("#{:03}", tracked_card.card.id);
                            let texture =
                                self.card_images.texture_for(context, tracked_card.card.id);
                            draw_tracker_control(
                                ui,
                                metrics.control_style,
                                texture,
                                &mut tracked_card.count,
                                Some(target),
                                title,
                                fallback_label,
                                Some(card_art_uv()),
                            );
                            tracker_index += 1;
                            if tracker_index % metrics.columns == 0 {
                                ui.end_row();
                            }
                        }

                        for tracked_group in &mut self.tracked_groups {
                            let title = tracked_group.title();
                            let fallback_label = tracked_group.fallback_label();
                            let image_uv = tracked_group.image_uv();
                            let texture = self.group_images.texture_for(
                                context,
                                &tracked_group.spec.id,
                                tracked_group.spec.image.as_deref(),
                            );
                            draw_tracker_control(
                                ui,
                                metrics.control_style,
                                texture,
                                &mut tracked_group.count,
                                None,
                                title,
                                fallback_label,
                                image_uv,
                            );
                            tracker_index += 1;
                            if tracker_index % metrics.columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        });
    }
}

fn sizes_are_close(left: egui::Vec2, right: egui::Vec2) -> bool {
    (left - right).abs().max_elem() < 2.0
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
}

impl TrackedGroup {
    fn title(&self) -> String {
        self.spec.name.clone()
    }

    fn fallback_label(&self) -> String {
        self.spec.name.clone()
    }

    fn image_uv(&self) -> Option<egui::Rect> {
        let image_path = self.spec.image.as_deref()?;
        let normalized_path = image_path.replace('\\', "/");

        if normalized_path.starts_with("assets/cards/") {
            Some(card_art_uv())
        } else {
            None
        }
    }
}
