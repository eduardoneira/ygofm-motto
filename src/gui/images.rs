use std::collections::HashMap;
use std::fs;

use eframe::egui::{self, TextureHandle};

#[derive(Default)]
pub(super) struct CardImageCache {
    textures: HashMap<u16, Option<TextureHandle>>,
}

#[derive(Default)]
pub(super) struct GroupImageCache {
    textures: HashMap<String, Option<TextureHandle>>,
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

impl GroupImageCache {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn texture_for(
        &mut self,
        context: &egui::Context,
        group_id: &str,
        image_path: Option<&str>,
    ) -> Option<&TextureHandle> {
        let image_path = image_path?;
        self.textures
            .entry(group_id.to_owned())
            .or_insert_with(|| load_image(context, format!("group-{group_id}"), image_path))
            .as_ref()
    }
}

fn load_card_image(context: &egui::Context, card_id: u16) -> Option<TextureHandle> {
    let image_path = format!("assets/cards/{card_id:03}.webp");
    load_image(context, format!("card-{card_id:03}"), image_path)
}

fn load_image(
    context: &egui::Context,
    texture_name: String,
    image_path: impl AsRef<std::path::Path>,
) -> Option<TextureHandle> {
    let image_bytes = fs::read(image_path).ok()?;
    let decoded_image = image::load_from_memory(&image_bytes).ok()?;
    let rgba_image = decoded_image.to_rgba8();
    let size = [rgba_image.width() as usize, rgba_image.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba_image.as_raw());

    Some(context.load_texture(texture_name, color_image, egui::TextureOptions::LINEAR))
}
