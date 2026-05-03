use eframe::egui::{self, TextureHandle};

use super::app::TrackedCard;

const CARD_ART_SIZE: egui::Vec2 = egui::vec2(132.0, 124.0);
const CARD_COUNTER_WIDTH: f32 = 24.0;
const CARD_COUNTER_BUTTON_SIZE: egui::Vec2 = egui::vec2(28.0, 24.0);

pub(super) fn draw_card_control(
    ui: &mut egui::Ui,
    texture: Option<&TextureHandle>,
    tracked_card: &mut TrackedCard,
    title: String,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(CARD_ART_SIZE.x, CARD_ART_SIZE.y + 34.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            tracked_card.count = tracked_card.count.min(tracked_card.target());

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
                        tracked_card.count < tracked_card.target(),
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
    match texture {
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
    }
}

fn card_art_uv() -> egui::Rect {
    egui::Rect::from_min_max(egui::pos2(0.12, 0.25), egui::pos2(0.90, 0.76))
}
