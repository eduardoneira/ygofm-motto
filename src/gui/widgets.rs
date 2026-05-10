use eframe::egui::{self, TextureHandle};

const CARD_ART_SIZE: egui::Vec2 = egui::vec2(132.0, 124.0);
const CARD_COUNTER_WIDTH: f32 = 24.0;
const CARD_COUNTER_BUTTON_SIZE: egui::Vec2 = egui::vec2(28.0, 24.0);

pub(super) fn draw_tracker_control(
    ui: &mut egui::Ui,
    texture: Option<&TextureHandle>,
    count: &mut u32,
    max_count: Option<u32>,
    title: String,
    fallback_label: String,
    image_uv: Option<egui::Rect>,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(CARD_ART_SIZE.x, CARD_ART_SIZE.y + 34.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            if let Some(max_count) = max_count {
                *count = (*count).min(max_count);
            }

            let response = draw_tracker_tile(ui, texture, fallback_label, image_uv);
            response.on_hover_text(title);

            ui.horizontal(|ui| {
                ui.set_width(CARD_ART_SIZE.x);
                let controls_width = CARD_COUNTER_BUTTON_SIZE.x * 2.0
                    + CARD_COUNTER_WIDTH
                    + ui.spacing().item_spacing.x * 2.0;
                ui.add_space((CARD_ART_SIZE.x - controls_width).max(0.0) / 2.0);

                if ui
                    .add_enabled(
                        *count > 0,
                        egui::Button::new("-").min_size(CARD_COUNTER_BUTTON_SIZE),
                    )
                    .clicked()
                {
                    *count -= 1;
                }

                ui.add_sized(
                    egui::vec2(CARD_COUNTER_WIDTH, CARD_COUNTER_BUTTON_SIZE.y),
                    egui::Label::new(count.to_string()),
                );

                if ui
                    .add_enabled(
                        max_count.is_none_or(|max_count| *count < max_count),
                        egui::Button::new("+").min_size(CARD_COUNTER_BUTTON_SIZE),
                    )
                    .clicked()
                {
                    *count += 1;
                }
            });
        },
    );
}

pub(super) fn card_art_uv() -> egui::Rect {
    egui::Rect::from_min_max(egui::pos2(0.12, 0.25), egui::pos2(0.90, 0.76))
}

fn draw_tracker_tile(
    ui: &mut egui::Ui,
    texture: Option<&TextureHandle>,
    fallback_label: String,
    image_uv: Option<egui::Rect>,
) -> egui::Response {
    match texture {
        Some(texture) => {
            let image = egui::Image::new((texture.id(), CARD_ART_SIZE));
            if let Some(image_uv) = image_uv {
                ui.add(image.uv(image_uv))
            } else {
                ui.add(image)
            }
        }
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
                compact_tile_label(&fallback_label),
                egui::TextStyle::Button.resolve(ui.style()),
                ui.visuals().weak_text_color(),
            );
            response
        }
    }
}

fn compact_tile_label(label: &str) -> String {
    const MAX_CHARS: usize = 16;

    if label.chars().count() <= MAX_CHARS {
        return label.to_owned();
    }

    label.chars().take(MAX_CHARS - 2).collect::<String>() + ".."
}
