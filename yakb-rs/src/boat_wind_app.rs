//! Boat wind animations

use std::f32::consts::PI;

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Frame, Pos2, Shape, Stroke, Vec2},
};

/// Unit circle
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct BoatWindApp {
    /// boat angle
    boat_angle_deg: f32,
    /// boat speed
    boat_speed: f32,
    /// wind angle
    wind_angle_deg: f32,
    /// current speed
    wind_speed: f32,
}

impl Default for BoatWindApp {
    fn default() -> Self {
        Self {
            boat_angle_deg: 45.0,
            boat_speed: 10.0,
            wind_angle_deg: 0.0,
            wind_speed: 10.0,
        }
    }
}

impl BoatWindApp {
    /// show the animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        let dt = ui.ctx().input(|i| i.stable_dt).min(0.05);

        // self.phase = (self.phase + dt * self.speed).rem_euclid(TAU);
        ui.ctx().request_repaint();

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ui, |ui| {
                self.paint_scene(ui, error_manager.is_debug());
                self.show_settings(ui);
            });
    }

    /// Show settings
    fn show_settings(&mut self, ui: &mut egui::Ui) {
        let panel_top_left = ui.min_rect().min;
        egui::Area::new(ui.id().with("scene_settings"))
            .fixed_pos(panel_top_left)
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(250))
                    .corner_radius(CornerRadius {
                        nw: 0,
                        ne: 0,
                        sw: 0,
                        se: 4,
                    })
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        ui.collapsing("Menu", |ui| {
                            ui.label("Wind");
                            ui.horizontal(|ui| {
                                ui.label("TWA");
                                ui.add(
                                    egui::Slider::new(&mut self.wind_angle_deg, -90.0..=90.0)
                                        .suffix("deg"),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("TWS");
                                ui.add(egui::Slider::new(&mut self.wind_speed, 0.0..=30.0));
                            });
                            ui.separator();
                            ui.label("Boat");
                            ui.horizontal(|ui| {
                                ui.label("Boat angle");
                                ui.add(
                                    egui::Slider::new(&mut self.boat_angle_deg, -90.0..=90.0)
                                        .suffix("deg"),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("Boat speed");
                                ui.add(egui::Slider::new(&mut self.boat_speed, 0.0..=30.0));
                            });
                            ui.separator();

                            if ui.button("Reset").clicked() {
                                self.boat_angle_deg = 0.0;
                                self.wind_angle_deg = 0.0;
                            }
                        });
                    });
            });
    }

    /// Paint scene
    fn paint_scene(&mut self, ui: &mut egui::Ui, _is_debug: bool) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let area = response.rect;
        let p = &painter;
        let bg = Color32::from_rgb(255, 255, 255);

        p.rect_filled(area, 0.0, bg);

        let circle_center = Pos2::new(area.center().x, area.top() + area.height() * 0.245);
        let painter = ui.painter();
        let boat_center = Pos2::new(circle_center.x, circle_center.y + 300.0);
        let boat_angle_rad = self.boat_angle_deg.to_radians() + (PI / 2.0);
        draw_boat(painter, boat_center, 0.7, boat_angle_rad);
        let end_boat_speed = boat_center
            - Vec2::new(
                self.boat_speed * 10.0 * boat_angle_rad.cos(),
                self.boat_speed * 10.0 * boat_angle_rad.sin(),
            );
        arrow_filled(
            painter,
            boat_center,
            end_boat_speed,
            Stroke::new(4.0, Color32::BLUE),
        );

        let wind_angle_rad = self.wind_angle_deg.to_radians() + (PI / 2.0);

        let wind_position = Pos2::new(area.center().x, area.top() + 100.0);

        let wind_vec = Vec2::angled(wind_angle_rad) * (self.wind_speed * 10.0);

        fat_arrow(
            painter,
            wind_position,
            wind_position + wind_vec,
            Color32::RED,
        );
        painter.text(
            wind_position,
            Align2::CENTER_TOP,
            "WIND",
            FontId::proportional(12.0),
            Color32::BLACK,
        );

        north_symbol(
            painter,
            Pos2::new(area.left() + (area.width() / 4.0), area.top() + 60.0),
        );

        let start_wind_boat_speed = end_boat_speed
            - Vec2::new(
                self.wind_speed * 10.0 * wind_angle_rad.cos(),
                self.wind_speed * 10.0 * wind_angle_rad.sin(),
            );
        // TWS
        arrow_filled(
            painter,
            start_wind_boat_speed,
            end_boat_speed,
            Stroke::new(4.0, Color32::RED),
        );

        // AWS
        arrow_filled(
            painter,
            start_wind_boat_speed,
            boat_center,
            Stroke::new(4.0, Color32::DARK_GREEN),
        );
    }
}

/// draw filled arrow
pub fn arrow_filled(painter: &egui::Painter, origin: Pos2, tip: Pos2, stroke: Stroke) {
    use egui::emath::Rot2;

    let vec = tip - origin;
    let len = vec.length();

    if len < f32::EPSILON {
        return;
    }

    let dir = vec / len;

    // Keep the arrowhead visible even for tiny vectors.
    let head_len = (len / 8.0).max(8.0);
    let head_width = (head_len * 0.5).max(stroke.width * 2.0);

    // Dynamic angle based on head dimensions.
    let angle = (head_width / head_len).atan();
    let rot = Rot2::from_angle(angle);

    let p1 = tip - head_len * (rot * dir);
    let p2 = tip - head_len * (rot.inverse() * dir);

    painter.line_segment([origin, tip], stroke);

    painter.add(Shape::convex_polygon(
        vec![tip, p1, p2],
        stroke.color,
        Stroke::NONE,
    ));
}

/// draw the boat
fn draw_boat(painter: &egui::Painter, origin: Pos2, scale: f32, angle: f32) {
    let points = [
        (-0.6, 9.1),
        (6.5, 11.2),
        (18.8, 34.9),
        (40.6, 91.1),
        (52.3, 126.4),
        (67.1, 187.4),
        (73.8, 243.5),
        (75.3, 293.9),
        (59.8, 426.0),
        (52.9, 446.2),
        (37.7, 455.1),
        (0.0, 460.4),
        (-37.7, 455.1),
        (-52.9, 446.2),
        (-59.8, 426.0),
        (-75.3, 293.9),
        (-73.8, 243.5),
        (-67.1, 187.4),
        (-52.3, 126.4),
        (-40.6, 91.1),
        (-18.8, 34.9),
        (-6.5, 11.2),
    ];
    let center_y = (9.1_f32).midpoint(460.4);
    let (sin, cos) = (angle - PI / 2.0).sin_cos();
    let vertices: Vec<Pos2> = points
        .iter()
        .map(|&(x, y)| {
            let y = y - center_y;

            // Rotate around the shape's center.
            let rotated_x = x * cos - y * sin;
            let rotated_y = x * sin + y * cos;
            Pos2::new(origin.x + rotated_x * scale, origin.y + rotated_y * scale)
        })
        .collect();

    painter.add(Shape::closed_line(
        vertices,
        Stroke::new(2.0, Color32::BLUE),
    ));
}

/// Draws a block arrow centered on `center`, pointing toward `target`.
/// Angle and length are both calculated from the two positions.
fn fat_arrow(painter: &egui::Painter, center: Pos2, target: Pos2, color: Color32) {
    let delta = target - center;
    let length = delta.length();
    if length < f32::EPSILON {
        return;
    }

    let rot = egui::emath::Rot2::from_angle(delta.y.atan2(delta.x));

    let head_length = length * 0.45;
    let head_half_width = length * 0.30;
    let shaft_half_width = length * 0.14;

    let half = length * 0.5;
    let neck_x = half - head_length;

    // Build the arrow pointing right around (0, 0), then rotate and move.
    let place = |x: f32, y: f32| center + rot * Vec2::new(x, y);

    // Shaft (extended 1px into the head to avoid a seam)
    painter.add(Shape::convex_polygon(
        vec![
            place(-half, -shaft_half_width),
            place(neck_x + 1.0, -shaft_half_width),
            place(neck_x + 1.0, shaft_half_width),
            place(-half, shaft_half_width),
        ],
        color,
        Stroke::NONE,
    ));

    // Head
    painter.add(Shape::convex_polygon(
        vec![
            place(neck_x, -head_half_width),
            place(half, 0.0),
            place(neck_x, head_half_width),
        ],
        color,
        Stroke::NONE,
    ));
}

/// Draws a north arrow (split kite with an "N" above it) centered on `pos`.
pub fn north_symbol(painter: &egui::Painter, pos: Pos2) {
    let size = 30.0; // half-height of the arrow; change to scale the symbol
    let half_width = size * 0.4;
    let outline = Stroke::new(1.5, Color32::BLACK);

    let tip = pos + Vec2::new(0.0, -size);
    let notch = pos + Vec2::new(0.0, size * 0.3);
    let bottom_left = pos + Vec2::new(-half_width, size * 0.6);
    let bottom_right = pos + Vec2::new(half_width, size * 0.6);

    // Left half: filled black
    painter.add(Shape::convex_polygon(
        vec![tip, bottom_left, notch],
        Color32::BLACK,
        outline,
    ));

    // Right half: white with a black outline
    painter.add(Shape::convex_polygon(
        vec![tip, notch, bottom_right],
        Color32::WHITE,
        outline,
    ));

    // "N" above the tip
    painter.text(
        tip + Vec2::new(0.0, -4.0),
        Align2::CENTER_BOTTOM,
        "N",
        FontId::proportional(size * 0.7),
        Color32::BLACK,
    );
}
