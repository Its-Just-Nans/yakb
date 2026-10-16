//! vector app

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Sense, Vec2},
    utils::grid::Grid,
};

use crate::utils::ui_settings;

/// Vector app
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct VectorApp {
    /// position
    pos: Pos2,
    /// vector
    vec: Vec2,
    /// second arrow
    second_arrow: (bool, Pos2, Vec2),
    /// grid
    grid: Grid,
    /// scene rect
    scene_rect: Rect,
}

impl Default for VectorApp {
    fn default() -> Self {
        let grid = Grid {
            spacing: Vec2::splat(10.0),
            ..Grid::default()
        };
        Self {
            pos: Pos2::new(0.0, 0.0),
            vec: Vec2::new(20.0, 30.0),
            second_arrow: (false, Pos2::new(10.0, 10.0), Vec2::new(20.0, 30.0)),
            grid,
            scene_rect: Rect::ZERO,
        }
    }
}

/// Draw a arrow
fn draw_arrow(
    ui: &mut egui::Ui,
    pos: &mut egui::Pos2,
    vec: &mut egui::Vec2,
    color: Color32,
    name: &str,
) {
    let end = *pos + *vec;

    let full_rect = Rect::from_two_pos(*pos, end).expand(10.0);
    ui.allocate_rect(full_rect, Sense::hover());

    let rect_start = Rect::from_center_size(*pos, Vec2::new(10.0, 10.0));
    let response_start = ui.interact(
        rect_start,
        ui.id().with(("start", name)),
        egui::Sense::drag(),
    );
    if response_start.dragged() {
        *pos += response_start.drag_delta();
    }

    let rect_end = egui::Rect::from_center_size(end, Vec2::new(10.0, 10.0));
    let response_end = ui.interact(rect_end, ui.id().with(("end", name)), egui::Sense::drag());
    if response_end.dragged() {
        *vec += response_end.drag_delta();
    }
    let end = *pos + *vec;

    let painter = ui.painter();

    let stroke = egui::Stroke::new(2.0, color);

    // Line direction.
    let dir = vec.normalized();

    // Perpendicular direction.
    let perp = egui::vec2(-dir.y, dir.x);

    // Arrowhead size.
    let head_len = 10.0;
    let head_width = 5.0;

    // Arrowhead points.
    let left = end - dir * head_len + perp * head_width;
    let right = end - dir * head_len - perp * head_width;

    // Main line.
    painter.line_segment([*pos, end], stroke);

    // Arrow head.
    painter.line_segment([end, left], stroke);

    painter.line_segment([end, right], stroke);
}

/// Color vec
const COLOR_VEC: Color32 = Color32::RED;
/// Color second vec
const COLOR_SECOND: Color32 = Color32::DARK_GREEN;

impl VectorApp {
    /// show the animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui, _error_manager: &mut ErrorManager) {
        if self.scene_rect.any_nan() {
            return;
        }
        egui::Scene::new()
            .zoom_range(0.0..=f32::INFINITY)
            .show(ui, &mut self.scene_rect, |ui| {
                let painter = ui.painter();
                let bg_r: egui::Response = ui.response();
                if bg_r.rect.is_finite() {
                    self.grid.draw(&bg_r.rect, painter);
                }
                draw_arrow(ui, &mut self.pos, &mut self.vec, COLOR_VEC, "base");
                if self.second_arrow.0 {
                    let (_, pos, vec) = &mut self.second_arrow;
                    draw_arrow(ui, pos, vec, COLOR_SECOND, "second");
                }
            });
        self.show_info(ui);
    }
    /// show info
    fn show_info(&mut self, ui: &mut egui::Ui) {
        ui_settings(ui, |ui| {
            ui.set_width(220.0);
            ui.horizontal(|ui| {
                ui.colored_label(COLOR_VEC, "pos: (");
                ui.add(egui::DragValue::new(&mut self.pos.x));
                ui.add(egui::DragValue::new(&mut self.pos.y));
                ui.colored_label(COLOR_VEC, ")");
            });
            ui.horizontal(|ui| {
                ui.colored_label(COLOR_VEC, "vec: (");
                ui.add(egui::DragValue::new(&mut self.vec.x));
                ui.add(egui::DragValue::new(&mut self.vec.y));
                ui.colored_label(COLOR_VEC, ")");
            });
            ui.checkbox(&mut self.second_arrow.0, "Second vector");
            if self.second_arrow.0 {
                let (_, pos, vec) = &mut self.second_arrow;
                ui.horizontal(|ui| {
                    ui.colored_label(COLOR_SECOND, "pos: (");
                    ui.add(egui::DragValue::new(&mut pos.x));
                    ui.add(egui::DragValue::new(&mut pos.y));
                    ui.colored_label(COLOR_SECOND, ")");
                });
                ui.horizontal(|ui| {
                    ui.colored_label(COLOR_SECOND, "vec: (");
                    ui.add(egui::DragValue::new(&mut vec.x));
                    ui.add(egui::DragValue::new(&mut vec.y));
                    ui.colored_label(COLOR_SECOND, ")");
                });
            }
        });
    }
}
