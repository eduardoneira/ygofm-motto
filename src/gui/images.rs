use std::collections::HashMap;
use std::fs;

use eframe::egui::{self, TextureHandle};

#[derive(Default)]
pub(super) struct CardImageCache {
    textures: HashMap<u16, Option<TextureHandle>>,
}

impl CardImageCache {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn texture_for(
        &mut self,
        context: &egui::Context,
        card_id: u16,
    ) -> Option<&TextureHandle> {
        self.textures
            .entry(card_id)
            .or_insert_with(|| load_card_image(context, card_id))
            .as_ref()
    }
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
