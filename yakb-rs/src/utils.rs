//! Utils functions

use bladvak::eframe::egui::{self, Align2, Color32, Pos2};

/// Draw text with background color
pub(crate) fn draw_text_with_background(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    text: String,
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
pub(crate) fn draw_text_with_background_align(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    align: Align2,
    text: String,
) {
    let galley = p.layout_no_wrap(text.clone(), font_id.clone(), background_color);
    let rect = align.anchor_size(pos_text, galley.size());
    p.rect_filled(rect, 0.0, background_color);
    p.text(pos_text, align, text, font_id.clone(), color_text);
}

/// Draw text with background and centered
pub(crate) fn draw_text_with_background_center(
    p: &egui::Painter,
    font_id: &egui::FontId,
    color_text: Color32,
    background_color: Color32,
    pos_text: Pos2,
    text: String,
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
