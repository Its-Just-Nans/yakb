//! vector app

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Pos2, Rect, Sense, Vec2},
    utils::grid::Grid,
};

/// Vector app
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct VectorApp {
    /// position
    pos: Pos2,
    /// vector
    vec: Vec2,
    /// grid
    grid: Grid,
    /// scene rect
    scene_rect: Rect,
}

impl Default for VectorApp {
    fn default() -> Self {
        Self {
            pos: Pos2::new(0.0, 0.0),
            vec: Vec2::new(20.0, 30.0),
            grid: Grid::default(),
            scene_rect: Rect::ZERO,
        }
    }
}

/// Draw a arrow
fn draw_arrow(ui: &mut egui::Ui, pos: &mut egui::Pos2, vec: &mut egui::Vec2) {
    let end = *pos + *vec;

    let full_rect = Rect::from_two_pos(*pos, end).expand(10.0);
    ui.allocate_rect(full_rect, Sense::hover());

    let rect_start = Rect::from_center_size(*pos, Vec2::new(10.0, 10.0));
    let response_start = ui.interact(rect_start, ui.id().with("start"), egui::Sense::drag());
    if response_start.dragged() {
        *pos += response_start.drag_delta();
    }

    let rect_end = egui::Rect::from_center_size(end, Vec2::new(10.0, 10.0));
    let response_end = ui.interact(rect_end, ui.id().with("end"), egui::Sense::drag());
    if response_end.dragged() {
        *vec += response_end.drag_delta();
    }

    let painter = ui.painter();

    let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);

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

/// show info
fn show_info(ui: &mut egui::Ui, pos: Pos2, vec: Vec2) {
    egui::Area::new(ui.id().with("scene_info"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            ui.set_width(220.0);
            egui::Frame::NONE
                .fill(egui::Color32::from_black_alpha(180))
                .corner_radius(4.0)
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    ui.set_width(200.0);
                    ui.monospace(format!("pos: ({:.1}, {:.1})", pos.x, pos.y));
                    ui.monospace(format!("vec: ({:.1}, {:.1})", vec.x, vec.y));
                });
        });
}

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
                draw_arrow(ui, &mut self.pos, &mut self.vec);
            });
        show_info(ui, self.pos, self.vec);
    }
}
