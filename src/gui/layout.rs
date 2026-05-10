use eframe::egui;

use super::widgets::{NATURAL_TRACKER_CONTROL_SIZE, TrackerControlStyle};

const NATURAL_GRID_SPACING: f32 = 14.0;
const WINDOW_PADDING: egui::Vec2 = egui::vec2(32.0, 40.0);
const MIN_TRACKER_SCALE: f32 = 0.65;

#[derive(Debug, Clone, Copy)]
pub(super) struct TrackerGridLayout {
    columns: usize,
    visible_rows: usize,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TrackerGridMetrics {
    pub(super) columns: usize,
    pub(super) spacing: egui::Vec2,
    pub(super) control_style: TrackerControlStyle,
    pub(super) compact_window_size: egui::Vec2,
}

impl TrackerGridLayout {
    pub(super) fn new(columns: usize, visible_rows: usize) -> Self {
        Self {
            columns: columns.max(1),
            visible_rows: visible_rows.max(1),
        }
    }

    pub(super) fn initial_window_size(&self) -> egui::Vec2 {
        self.natural_content_size() + WINDOW_PADDING
    }

    pub(super) fn metrics_for_available_size(
        &self,
        available_size: egui::Vec2,
    ) -> TrackerGridMetrics {
        let scale = self.scale_for_available_size(available_size);

        let control_style = TrackerControlStyle::scaled(scale);
        let spacing = egui::Vec2::splat(NATURAL_GRID_SPACING * scale);
        let compact_content_size = grid_size(
            control_style.control_size(),
            spacing.x,
            self.columns,
            self.visible_rows,
        );

        TrackerGridMetrics {
            columns: self.columns,
            spacing,
            control_style,
            compact_window_size: compact_content_size + WINDOW_PADDING,
        }
    }

    fn scale_for_available_size(&self, available_size: egui::Vec2) -> f32 {
        let natural_content_size = self.natural_content_size();

        if natural_content_size.x <= 0.0 || natural_content_size.y <= 0.0 {
            return 1.0;
        }

        let width_scale = scale_for_axis(available_size.x, natural_content_size.x);
        let height_scale = scale_for_axis(available_size.y, natural_content_size.y);

        width_scale.min(height_scale).clamp(MIN_TRACKER_SCALE, 1.0)
    }

    fn natural_content_size(&self) -> egui::Vec2 {
        grid_size(
            NATURAL_TRACKER_CONTROL_SIZE,
            NATURAL_GRID_SPACING,
            self.columns,
            self.visible_rows,
        )
    }
}

fn scale_for_axis(available: f32, natural: f32) -> f32 {
    if available.is_finite() && available > 0.0 {
        available / natural
    } else {
        1.0
    }
}

fn grid_size(control_size: egui::Vec2, spacing: f32, columns: usize, rows: usize) -> egui::Vec2 {
    let columns = columns.max(1) as f32;
    let rows = rows.max(1) as f32;

    egui::vec2(
        control_size.x * columns + spacing * (columns - 1.0),
        control_size.y * rows + spacing * (rows - 1.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_natural_scale_when_the_grid_fits() {
        let layout = TrackerGridLayout::new(5, 3);
        let metrics = layout.metrics_for_available_size(egui::vec2(1000.0, 1000.0));

        assert_eq!(metrics.control_style.scale(), 1.0);
    }

    #[test]
    fn scales_down_to_fit_requested_rows_and_columns() {
        let layout = TrackerGridLayout::new(5, 3);
        let metrics = layout.metrics_for_available_size(layout.initial_window_size() / 2.0);

        assert!(metrics.control_style.scale() < 1.0);
        assert!(metrics.control_style.scale() >= MIN_TRACKER_SCALE);
    }

    #[test]
    fn compact_window_size_shrinks_with_the_scale() {
        let layout = TrackerGridLayout::new(5, 6);
        let metrics = layout.metrics_for_available_size(layout.initial_window_size() / 2.0);

        assert!(metrics.compact_window_size.x < layout.initial_window_size().x);
        assert!(metrics.compact_window_size.y < layout.initial_window_size().y);
    }

    #[test]
    fn never_scales_below_the_readable_minimum() {
        let layout = TrackerGridLayout::new(20, 20);
        let metrics = layout.metrics_for_available_size(egui::vec2(100.0, 100.0));

        assert_eq!(metrics.control_style.scale(), MIN_TRACKER_SCALE);
    }
}
