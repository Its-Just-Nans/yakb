//! Utils functions

use bladvak::eframe::egui::{self, Align2, Color32, CornerRadius, Pos2};

/// Draw text with background color
pub(crate) fn draw_text_with_background<S: Into<String>>(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    text: S,
) {
    draw_text_with_background_align(
        p,
        font_id,
        color_text,
        background_color,
        pos_text,
        Align2::LEFT_TOP,
        text,
    );
}

/// Draw text with background and align param
pub(crate) fn draw_text_with_background_align<S: Into<String>>(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    align: Align2,
    text: S,
) {
    let text = text.into();
    let galley = p.layout_no_wrap(text.clone(), font_id.clone(), background_color);
    let rect = align.anchor_size(pos_text, galley.size());
    p.rect_filled(rect, 0.0, background_color);
    p.text(pos_text, align, text, font_id.clone(), color_text);
}

/// Draw text with background and centered
pub(crate) fn draw_text_with_background_center<S: Into<String>>(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    text: S,
) {
    draw_text_with_background_align(
        p,
        font_id,
        color_text,
        background_color,
        pos_text,
        Align2::CENTER_TOP,
        text,
    );
}

/// Ui settings (top left)
pub(crate) fn ui_settings(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_top_left = ui.min_rect().min;
    let is_dark_theme = ui.ctx().global_style().visuals.dark_mode;
    let background_color = if is_dark_theme {
        Color32::from_black_alpha(250)
    } else {
        Color32::from_white_alpha(250)
    };
    egui::Area::new(ui.id().with("scene_settings"))
        .fixed_pos(panel_top_left)
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            let response = egui::Frame::NONE
                .fill(background_color)
                .corner_radius(CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 0,
                    se: 4,
                })
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    add_contents(ui);
                });
            let rect = response.response.rect;
            let stroke = egui::Stroke::new(1.0, egui::Color32::GRAY);
            if !is_dark_theme {
                ui.painter()
                    .line_segment([rect.right_top(), rect.right_bottom()], stroke);

                ui.painter()
                    .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);
            }
        });
}
