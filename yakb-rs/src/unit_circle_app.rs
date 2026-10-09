//! Unit Circle

use bladvak::eframe::egui::{self, Color32, Frame, Pos2, Rect, Stroke, Vec2};
use std::f32::consts::TAU;

/// Unit circle
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct UnitCircleApp {
    /// current phase
    phase: f32,
    /// current speed
    speed: f32,
    /// is animation running
    running: bool,
    /// show projections
    show_projections: bool,
    /// show sin
    show_sin: bool,
    /// show cos
    show_cos: bool,
}

impl Default for UnitCircleApp {
    fn default() -> Self {
        Self {
            phase: 0.0,
            speed: 1.0,
            running: true,
            show_projections: true,
            show_sin: true,
            show_cos: true,
        }
    }
}

impl UnitCircleApp {
    /// show the animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        let dt = ui.ctx().input(|i| i.stable_dt).min(0.05);

        egui::panel::Panel::top("controls").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.running, "Animate");
                ui.checkbox(&mut self.show_projections, "Show projections");
                ui.checkbox(&mut self.show_cos, "Cos");
                ui.checkbox(&mut self.show_sin, "Sin");
                ui.add_space(12.0);
                ui.label("Speed");
                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0).suffix("×"));
                if ui.button("Reset").clicked() {
                    self.phase = 0.0;
                }
            });
        });

        if self.running {
            self.phase = (self.phase + dt * self.speed).rem_euclid(TAU);
            ui.ctx().request_repaint();
        }

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ui, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                paint_scene(
                    &painter,
                    response.rect,
                    self.phase,
                    self.show_projections,
                    self.show_sin,
                    self.show_cos,
                );
            });
    }
}

/// Color of the sin
const COLOR_SIN: Color32 = Color32::RED;
/// Color of the cos
const COLOR_COS: Color32 = Color32::DARK_GREEN;
/// Color phase
const COLOR_PHASE: Color32 = Color32::DARK_GRAY;

/// Paint circle and axis
fn paint_circle_and_axis(
    p: &egui::Painter,
    area: Rect,
    circle_center: Pos2,
    radius: f32,
    phase: f32,
) {
    let axis = Color32::from_rgb(65, 65, 65);
    let blue = Color32::from_rgb(28, 40, 180);
    p.line_segment(
        [
            Pos2::new(area.left(), circle_center.y),
            Pos2::new(area.right(), circle_center.y),
        ],
        Stroke::new(1.0, axis),
    );
    // vertical axis
    p.line_segment(
        [
            Pos2::new(circle_center.x, area.top()),
            Pos2::new(circle_center.x, area.bottom()),
        ],
        Stroke::new(1.0, axis),
    );

    // Dashed unit circle.
    let n = 180;
    #[allow(clippy::cast_precision_loss)]
    for i in 0..n {
        if i % 2 == 0 {
            let a0 = TAU * i as f32 / n as f32;
            let a1 = TAU * (i + 1) as f32 / n as f32;
            let q0 = Pos2::new(
                circle_center.x + radius * a0.cos(),
                circle_center.y - radius * a0.sin(),
            );
            let q1 = Pos2::new(
                circle_center.x + radius * a1.cos(),
                circle_center.y - radius * a1.sin(),
            );
            p.line_segment([q0, q1], Stroke::new(1.8, blue));
        }
    }
    p.line_segment(
        [
            Pos2::new(circle_center.x + radius * phase.cos(), circle_center.y),
            Pos2::new(
                circle_center.x + radius * phase.cos(),
                circle_center.y - radius * phase.sin(),
            ),
        ],
        Stroke::new(2.0, COLOR_SIN),
    );
    p.line_segment(
        [
            Pos2::new(
                circle_center.x + radius * phase.cos(),
                circle_center.y - radius * phase.sin(),
            ),
            Pos2::new(circle_center.x, circle_center.y - radius * phase.sin()),
        ],
        Stroke::new(2.0, COLOR_COS),
    );
}

/// draw angle
fn draw_angle(p: &egui::Painter, circle_center: Pos2, radius: f32, phase: f32) {
    let nb_points = 100;
    #[allow(clippy::cast_precision_loss)]
    let points: Vec<Pos2> = (0..=nb_points)
        .map(|i| {
            let angle = phase * i as f32 / nb_points as f32;
            circle_center + egui::vec2(angle.cos(), -angle.sin()) * (radius / 2.0)
        })
        .collect();

    p.add(egui::Shape::line(points, Stroke::new(2.0, COLOR_PHASE)));
}
/// Paint the scene
fn paint_scene(
    p: &egui::Painter,
    rect: Rect,
    phase: f32,
    show_projection: bool,
    show_sin: bool,
    show_cos: bool,
) {
    let bg = Color32::from_rgb(255, 255, 255);
    let blue = Color32::from_rgb(28, 40, 180);
    let axis = Color32::from_rgb(65, 65, 65);
    let guide = Color32::from_rgb(100, 110, 205);

    p.rect_filled(rect, 0.0, bg);

    let pad = 18.0;
    let area = Rect::from_min_max(rect.min + Vec2::splat(pad), rect.max - Vec2::splat(pad));
    let circle_center = Pos2::new(area.center().x, area.top() + area.height() * 0.245);
    let radius = (area.width() * 0.42).min(area.height() * 0.205).max(25.0);

    paint_circle_and_axis(p, area, circle_center, radius, phase);

    let point_on_circle = Pos2::new(
        circle_center.x + radius * phase.cos(),
        circle_center.y - radius * phase.sin(),
    );
    p.line_segment(
        [Pos2::new(circle_center.x, circle_center.y), point_on_circle],
        Stroke::new(2.0, blue),
    );
    p.circle_filled(point_on_circle, 3.8, blue);
    p.circle_filled(Pos2::new(circle_center.x, circle_center.y), 2.2, axis);

    if show_cos {
        let wave_top = circle_center.y + radius + 18.0;
        let wave_bottom = area.bottom();
        let wave_height = (wave_bottom - wave_top).max(40.0);
        // axis
        p.line_segment(
            [
                Pos2::new(area.left(), wave_top),
                Pos2::new(area.right(), wave_top),
            ],
            Stroke::new(1.0, axis),
        );
        paint_cos(
            p,
            phase,
            circle_center.x,
            radius,
            wave_top,
            wave_height,
            blue,
        );
        let wave_point = Pos2::new(circle_center.x + radius * phase.cos(), wave_top);
        p.circle_filled(wave_point, 3.8, blue);
        p.line_segment(
            [Pos2::new(circle_center.x, wave_top), wave_point],
            Stroke::new(2.0, COLOR_COS),
        );
        if show_projection {
            let projection_bottom = Pos2::new(point_on_circle.x, wave_top);
            let projection_top =
                Pos2::new(point_on_circle.x, point_on_circle.y.max(circle_center.y));
            dashed_line(p, projection_top, projection_bottom, 4.0, 3.0, guide);
        }
    }
    if show_sin {
        let graph_left = circle_center.x + radius + 18.0;
        let graph_right = area.right();
        let graph_width = (graph_right - graph_left).max(40.0);
        let wave_left = circle_center.x + radius + 18.0;
        p.line_segment(
            [
                Pos2::new(wave_left, area.top()),
                Pos2::new(wave_left, area.bottom()),
            ],
            Stroke::new(1.0, axis),
        );

        paint_sin(
            p,
            phase,
            wave_left,
            radius,
            circle_center.y,
            graph_width,
            blue,
        );
        let wave_point = Pos2::new(wave_left, circle_center.y - radius * phase.sin());
        p.circle_filled(wave_point, 3.8, blue);
        p.line_segment(
            [Pos2::new(wave_left, circle_center.y), wave_point],
            Stroke::new(2.0, COLOR_SIN),
        );
        if show_projection {
            let projection_right = Pos2::new(wave_left, point_on_circle.y);
            let projection_left =
                Pos2::new(point_on_circle.x.max(circle_center.x), point_on_circle.y);
            dashed_line(p, projection_left, projection_right, 4.0, 3.0, guide);
        }
    }
    draw_angle(p, circle_center, radius, phase);
    paint_text(p, phase, circle_center, radius);
}

/// Pain the texts
fn paint_text(p: &egui::Painter, phase: f32, circle_center: Pos2, radius: f32) {
    let font_id = egui::FontId::proportional(13.0);

    let text = format!("sin θ = {:.2}", phase.sin());
    let galley = p.layout_no_wrap(text.clone(), font_id.clone(), Color32::WHITE);
    let pos_text = Pos2::new(circle_center.x + radius + 24.0, circle_center.y);
    p.rect_filled(
        Rect::from_min_max(pos_text, pos_text + galley.size()),
        0.0,
        Color32::WHITE,
    );
    p.text(
        pos_text,
        egui::Align2::LEFT_TOP,
        text,
        font_id.clone(),
        COLOR_SIN,
    );

    let text = format!("cos θ = {:.2}", phase.cos());
    let galley = p.layout_no_wrap(text.clone(), font_id.clone(), Color32::WHITE);

    let pos_text = Pos2::new(circle_center.x - 25.0, circle_center.y + radius + 30.0);
    p.rect_filled(
        Rect::from_min_max(pos_text, pos_text + galley.size()),
        0.0,
        Color32::WHITE,
    );
    p.text(
        pos_text,
        egui::Align2::LEFT_TOP,
        text,
        font_id.clone(),
        COLOR_COS,
    );

    let text = format!("θ = {phase:.2}");
    let galley = p.layout_no_wrap(text.clone(), font_id.clone(), Color32::WHITE);

    let pos_text = Pos2::new(circle_center.x + 10.0, circle_center.y + 10.0);
    p.rect_filled(
        Rect::from_min_max(pos_text, pos_text + galley.size()),
        0.0,
        Color32::WHITE,
    );
    p.text(pos_text, egui::Align2::LEFT_TOP, text, font_id, COLOR_PHASE);
}

/// show the cos
fn paint_cos(
    p: &egui::Painter,
    phase: f32,
    circle_center_x: f32,
    radius: f32,
    wave_top: f32,
    wave_height: f32,
    blue: Color32,
) {
    let segments = 300;
    #[allow(clippy::cast_precision_loss)]
    for i in 0..segments {
        let t1 = i as f32 / segments as f32;
        let t2 = (i + 1) as f32 / segments as f32;
        p.line_segment(
            [
                Pos2::new(
                    circle_center_x + radius * (phase + t1 * TAU).cos(),
                    wave_top + t1 * wave_height,
                ),
                Pos2::new(
                    circle_center_x + radius * (phase + t2 * TAU).cos(),
                    wave_top + t2 * wave_height,
                ),
            ],
            Stroke::new(2.2, blue),
        );
    }
}

/// show the sin
fn paint_sin(
    p: &egui::Painter,
    phase: f32,
    graph_left: f32,
    radius: f32,
    circle_center_y: f32,
    graph_width: f32,
    blue: Color32,
) {
    let segments = 300;
    #[allow(clippy::cast_precision_loss)]
    for i in 0..segments {
        let t1 = i as f32 / segments as f32;
        let t2 = (i + 1) as f32 / segments as f32;

        let angle1 = phase + t1 * TAU;
        let angle2 = phase + t2 * TAU;

        let p1 = Pos2::new(
            graph_left + t1 * graph_width,
            circle_center_y - radius * angle1.sin(),
        );

        let p2 = Pos2::new(
            graph_left + t2 * graph_width,
            circle_center_y - radius * angle2.sin(),
        );

        p.line_segment([p1, p2], Stroke::new(2.2, blue));
    }
}

/// draw a dashed line
fn dashed_line(p: &egui::Painter, from: Pos2, to: Pos2, dash: f32, gap: f32, color: Color32) {
    let delta = to - from;
    let len = delta.length();
    if len <= 0.0 {
        return;
    }
    let dir = delta / len;
    let mut at = 0.0;
    while at < len {
        let end = (at + dash).min(len);
        p.line_segment([from + dir * at, from + dir * end], Stroke::new(1.3, color));
        at += dash + gap;
    }
}
