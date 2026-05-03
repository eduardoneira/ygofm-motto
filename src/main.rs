use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

use eframe::egui::{self, TextureHandle};
use ygofm_motto::{Card, CardDatabase, TrackedCardSpec, bundled_tracked_card_specs};

const CARD_ART_SIZE: egui::Vec2 = egui::vec2(132.0, 124.0);
const CARD_COUNTER_WIDTH: f32 = 24.0;
const CARD_COUNTER_BUTTON_SIZE: egui::Vec2 = egui::vec2(28.0, 24.0);
const MAX_CARD_COUNT: u32 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|argument| argument == "--cli") {
        return run_cli();
    }

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
struct TrackedCard {
    spec: TrackedCardSpec,
    card: Card,
    count: u32,
}

struct CardTrackerApp {
    tracked_cards: Vec<TrackedCard>,
    card_images: HashMap<u16, Option<TextureHandle>>,
    missing_card_ids: Vec<u16>,
    load_error: Option<String>,
}

impl CardTrackerApp {
    fn from_database(database: CardDatabase) -> Self {
        let mut missing_card_ids = Vec::new();

        let specs = match bundled_tracked_card_specs() {
            Ok(specs) => specs,
            Err(error) => {
                return Self {
                    tracked_cards: Vec::new(),
                    card_images: HashMap::new(),
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
            card_images: HashMap::new(),
            missing_card_ids,
            load_error: None,
        }
    }

    fn ensure_card_images_loaded(&mut self, context: &egui::Context) {
        for tracked_card in &self.tracked_cards {
            let card_id = tracked_card.card.id;
            if !self.card_images.contains_key(&card_id) {
                self.card_images
                    .insert(card_id, load_card_image(context, card_id));
            }
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

            self.ensure_card_images_loaded(context);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(14.0, 14.0);

                    for tracked_card in &mut self.tracked_cards {
                        let title = tracked_card.title();
                        let texture = self
                            .card_images
                            .get(&tracked_card.card.id)
                            .and_then(Option::as_ref);
                        draw_card_control(ui, texture, tracked_card, title);
                    }
                });
            });
        });
    }
}

fn draw_card_control(
    ui: &mut egui::Ui,
    texture: Option<&TextureHandle>,
    tracked_card: &mut TrackedCard,
    title: String,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(CARD_ART_SIZE.x, CARD_ART_SIZE.y + 34.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            tracked_card.count = tracked_card.count.min(MAX_CARD_COUNT);

            let response = draw_card_tile(ui, texture, tracked_card);
            response.on_hover_text(title);

            ui.horizontal(|ui| {
                ui.set_width(CARD_ART_SIZE.x);
                let controls_width = CARD_COUNTER_BUTTON_SIZE.x * 2.0
                    + CARD_COUNTER_WIDTH
                    + ui.spacing().item_spacing.x * 2.0;
                ui.add_space((CARD_ART_SIZE.x - controls_width).max(0.0) / 2.0);

                if ui
                    .add_enabled(
                        tracked_card.count > 0,
                        egui::Button::new("-").min_size(CARD_COUNTER_BUTTON_SIZE),
                    )
                    .clicked()
                {
                    tracked_card.count -= 1;
                }

                ui.add_sized(
                    egui::vec2(CARD_COUNTER_WIDTH, CARD_COUNTER_BUTTON_SIZE.y),
                    egui::Label::new(tracked_card.count_label()),
                );

                if ui
                    .add_enabled(
                        tracked_card.count < MAX_CARD_COUNT,
                        egui::Button::new("+").min_size(CARD_COUNTER_BUTTON_SIZE),
                    )
                    .clicked()
                {
                    tracked_card.count += 1;
                }
            });
        },
    );
}

fn draw_card_tile(
    ui: &mut egui::Ui,
    texture: Option<&TextureHandle>,
    tracked_card: &TrackedCard,
) -> egui::Response {
    let response = match texture {
        Some(texture) => ui.add(egui::Image::new((texture.id(), CARD_ART_SIZE)).uv(card_art_uv())),
        None => {
            let (rect, response) = ui.allocate_exact_size(CARD_ART_SIZE, egui::Sense::hover());
            ui.painter()
                .rect_filled(rect, 4.0, ui.visuals().faint_bg_color);
            ui.painter().rect_stroke(
                rect,
                4.0,
                egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("#{:03}", tracked_card.card.id),
                egui::TextStyle::Button.resolve(ui.style()),
                ui.visuals().weak_text_color(),
            );
            response
        }
    };

    response
}

fn card_art_uv() -> egui::Rect {
    egui::Rect::from_min_max(egui::pos2(0.12, 0.25), egui::pos2(0.90, 0.76))
}

fn load_card_image(context: &egui::Context, card_id: u16) -> Option<TextureHandle> {
    let image_path = format!("assets/cards/{card_id:03}.webp");
    let image_bytes = fs::read(image_path).ok()?;
    let decoded_image = image::load_from_memory(&image_bytes).ok()?;
    let rgba_image = decoded_image.to_rgba8();
    let size = [rgba_image.width() as usize, rgba_image.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba_image.as_raw());

    Some(context.load_texture(
        format!("card-{card_id:03}"),
        color_image,
        egui::TextureOptions::LINEAR,
    ))
}

impl TrackedCard {
    fn title(&self) -> String {
        self.spec
            .label
            .clone()
            .unwrap_or_else(|| format!("#{:03} {}", self.card.id, self.card.name))
    }

    fn count_label(&self) -> String {
        self.count.to_string()
    }
}

fn run_cli() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;

    println!(
        "Loaded {} Yu-Gi-Oh! Forbidden Memories cards and {} duelists.",
        database.len(),
        database.duelists().len()
    );
    print_help();

    loop {
        print!("command> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input == "q" {
            break;
        }

        if input == "help" {
            print_help();
            continue;
        }

        let parts = input.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["duelists"] => {
                for duelist in database.duelists() {
                    println!("#{:02} {}", duelist.id, duelist.name);
                }
            }
            ["card", card_input] => {
                let card_number = match card_input.parse::<u16>() {
                    Ok(card_number) => card_number,
                    Err(_) => {
                        println!("Please enter a card number from 1 to 722.");
                        continue;
                    }
                };

                match database.card(card_number) {
                    Some(card) => println!("{}", database.format_card_details(card)),
                    None => {
                        println!("No card found for #{card_number:03}. Try a number from 1 to 722.")
                    }
                }
            }
            ["duelist", duelist_input] => {
                let duelist_number = match duelist_input.parse::<u8>() {
                    Ok(duelist_number) => duelist_number,
                    Err(_) => {
                        println!("Please enter a duelist number from 1 to 39.");
                        continue;
                    }
                };

                match database.duelist(duelist_number) {
                    Some(duelist) => println!("{}", database.format_duelist_details(duelist)),
                    None => println!(
                        "No duelist found for #{duelist_number:02}. Try a number from 1 to 39."
                    ),
                }
            }
            _ => {
                println!("Unknown command.");
                print_help();
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Commands:");
    println!("  card <number>     Show card details, for example card 35");
    println!("  duelist <number>  Show opponent deck and drop pools, for example duelist 1");
    println!("  duelists          List all duelists");
    println!("  q                 Quit");
}
