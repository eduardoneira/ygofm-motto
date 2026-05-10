use eframe::egui::{self, TextureHandle};

const CARD_ART_SIZE: egui::Vec2 = egui::vec2(132.0, 124.0);
const CARD_COUNTER_WIDTH: f32 = 24.0;
const CARD_COUNTER_BUTTON_SIZE: egui::Vec2 = egui::vec2(28.0, 24.0);
pub(super) const NATURAL_TRACKER_CONTROL_SIZE: egui::Vec2 =
    egui::vec2(CARD_ART_SIZE.x, CARD_ART_SIZE.y + 34.0);
const CARD_COUNTER_TEXT_SIZE: f32 = 14.0;

#[derive(Debug, Clone, Copy)]
pub(super) struct TrackerControlStyle {
    art_size: egui::Vec2,
    counter_width: f32,
    counter_button_size: egui::Vec2,
    control_size: egui::Vec2,
    scale: f32,
}

impl TrackerControlStyle {
    pub(super) fn scaled(scale: f32) -> Self {
        Self {
            art_size: CARD_ART_SIZE * scale,
            counter_width: CARD_COUNTER_WIDTH * scale,
            counter_button_size: CARD_COUNTER_BUTTON_SIZE * scale,
            control_size: NATURAL_TRACKER_CONTROL_SIZE * scale,
            scale,
        }
    }

    #[cfg(test)]
    pub(super) fn scale(&self) -> f32 {
        self.scale
    }
}

pub(super) fn draw_tracker_control(
    ui: &mut egui::Ui,
    style: TrackerControlStyle,
    texture: Option<&TextureHandle>,
    count: &mut u32,
    max_count: Option<u32>,
    title: String,
    fallback_label: String,
    image_uv: Option<egui::Rect>,
) {
    ui.allocate_ui_with_layout(
        style.control_size,
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            if let Some(max_count) = max_count {
                *count = (*count).min(max_count);
            }

            let response = draw_tracker_tile(ui, style, texture, fallback_label, image_uv);
            response.on_hover_text(title);

            ui.horizontal(|ui| {
                ui.set_width(style.art_size.x);
                let controls_width = style.counter_button_size.x * 2.0
                    + style.counter_width
                    + ui.spacing().item_spacing.x * 2.0;
                ui.add_space((style.art_size.x - controls_width).max(0.0) / 2.0);

                if ui
                    .add_enabled(
                        *count > 0,
                        egui::Button::new(scaled_text("-", style.scale))
                            .min_size(style.counter_button_size),
                    )
                    .clicked()
                {
                    *count -= 1;
                }

                ui.add_sized(
                    egui::vec2(style.counter_width, style.counter_button_size.y),
                    egui::Label::new(scaled_text(count.to_string(), style.scale)),
                );

                if ui
                    .add_enabled(
                        max_count.is_none_or(|max_count| *count < max_count),
                        egui::Button::new(scaled_text("+", style.scale))
                            .min_size(style.counter_button_size),
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
    style: TrackerControlStyle,
    texture: Option<&TextureHandle>,
    fallback_label: String,
    image_uv: Option<egui::Rect>,
) -> egui::Response {
    match texture {
        Some(texture) => {
            let image = egui::Image::new((texture.id(), style.art_size));
            if let Some(image_uv) = image_uv {
                ui.add(image.uv(image_uv))
            } else {
                ui.add(image)
            }
        }
        None => {
            let (rect, response) = ui.allocate_exact_size(style.art_size, egui::Sense::hover());
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
                egui::FontId::proportional(CARD_COUNTER_TEXT_SIZE * style.scale),
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

fn scaled_text(text: impl Into<String>, scale: f32) -> egui::RichText {
    egui::RichText::new(text).size(CARD_COUNTER_TEXT_SIZE * scale)
}
