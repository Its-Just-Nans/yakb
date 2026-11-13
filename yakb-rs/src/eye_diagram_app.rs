//! Eye diagram

use bladvak::{ErrorManager, eframe::egui};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};

/// Eye Diagram app
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct EyeDiagramApp {
    signal: Vec<f32>,
    // Last mouse position, used for interpolation while drawing.
    last_drag: Option<(usize, f32)>,
}

const PERIODS: usize = 4;
const SAMPLES_PER_PERIOD: usize = 100;
const SAMPLES: usize = PERIODS * SAMPLES_PER_PERIOD + 1;

impl Default for EyeDiagramApp {
    #[allow(clippy::cast_precision_loss)]
    fn default() -> Self {
        let signal = (0..SAMPLES)
            .map(|i| {
                let t = i as f32 / (SAMPLES - 1) as f32;

                0.65 * (std::f32::consts::TAU * (t * 1.25 - 0.15)).sin()
                    + 0.25 * (std::f32::consts::TAU * (t * 3.1 + 0.1)).sin()
            })
            .collect();

        Self {
            signal,
            last_drag: None,
        }
    }
}

/// Draw the grid
#[allow(clippy::cast_precision_loss)]
fn draw_grid(painter: &egui::Painter, plot: Rect, divisions: usize, labels: &[(&str, f32)]) {
    let grid = Color32::from_gray(190);

    for i in 0..=4 {
        let y = plot.top() + i as f32 / 4.0 * plot.height();

        painter.line_segment(
            [Pos2::new(plot.left(), y), Pos2::new(plot.right(), y)],
            Stroke::new(0.6, grid),
        );
    }

    for i in 0..=divisions {
        let x = plot.left() + i as f32 / divisions as f32 * plot.width();

        painter.line_segment(
            [Pos2::new(x, plot.top()), Pos2::new(x, plot.bottom())],
            Stroke::new(0.6, grid),
        );
    }

    painter.rect_stroke(
        plot,
        0.0,
        Stroke::new(0.8, Color32::GRAY),
        egui::StrokeKind::Inside,
    );

    for (label, x) in labels {
        painter.text(
            Pos2::new(plot.left() + x * plot.width(), plot.bottom() + 8.0),
            Align2::CENTER_TOP,
            *label,
            FontId::proportional(13.0),
            Color32::GRAY,
        );
    }
}

impl EyeDiagramApp {
    pub(crate) fn show(&mut self, ui: &mut egui::Ui, _error_manager: &mut ErrorManager) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.ui(ui);
        });
    }

    /// Show the ui
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_lossless)]
    fn ui(&mut self, ui: &mut egui::Ui) {
        const Y_MIN: f32 = -1.1;
        const Y_MAX: f32 = 1.1;

        if self.signal.len() != SAMPLES {
            let old = self.signal.clone();

            self.signal = (0..SAMPLES)
                .map(|i| {
                    if old.is_empty() {
                        return 0.0;
                    }

                    if old.len() == 1 {
                        return old[0];
                    }

                    let x = i as f32 * (old.len() - 1) as f32 / (SAMPLES - 1) as f32;

                    let a = x.floor() as usize;
                    let b = (a + 1).min(old.len() - 1);
                    let t = x.fract();

                    old[a] * (1.0 - t) + old[b] * t
                })
                .collect();
        }

        let color_at = |t: f32| -> Color32 {
            let stops = [
                Color32::from_rgb(0, 55, 255),
                Color32::from_rgb(0, 220, 255),
                Color32::from_rgb(0, 235, 60),
                Color32::from_rgb(255, 225, 0),
                Color32::from_rgb(255, 35, 0),
            ];

            let t = t.clamp(0.0, 1.0) * 4.0;

            let i = (t.floor() as usize).min(3);
            let frac = t.fract();

            let a = stops[i];
            let b = stops[i + 1];

            Color32::from_rgb(
                (a.r() as f32 * (1.0 - frac) + b.r() as f32 * frac) as u8,
                (a.g() as f32 * (1.0 - frac) + b.g() as f32 * frac) as u8,
                (a.b() as f32 * (1.0 - frac) + b.b() as f32 * frac) as u8,
            )
        };

        let point = |plot: Rect, x: f32, y: f32| -> Pos2 {
            Pos2::new(
                plot.left() + x * plot.width(),
                plot.bottom() - (y - Y_MIN) / (Y_MAX - Y_MIN) * plot.height(),
            )
        };

        let value_from_y = |plot: Rect, y: f32| -> f32 {
            Y_MIN + (plot.bottom() - y) / plot.height() * (Y_MAX - Y_MIN)
        };

        ui.heading("Signal — 4 periods");
        ui.label("Click and drag anywhere to draw the signal.");

        let width = ui.available_width();
        let height = 270.0;

        let (response, painter) =
            ui.allocate_painter(Vec2::new(width, height), Sense::click_and_drag());

        let rect = response.rect;

        let plot = Rect::from_min_max(
            rect.min + Vec2::new(42.0, 10.0),
            rect.max - Vec2::new(12.0, 27.0),
        );

        draw_grid(
            &painter,
            plot,
            20,
            &[
                ("0", 0.0),
                ("d", 0.25),
                ("2d", 0.50),
                ("3d", 0.75),
                ("4d", 1.0),
            ],
        );

        // Y labels
        for i in 0..=4 {
            let v = -1.0 + i as f32 * 0.5;

            painter.text(
                Pos2::new(plot.left() - 8.0, point(plot, 0.0, v).y),
                Align2::RIGHT_CENTER,
                format!("{v:.1}"),
                FontId::proportional(10.0),
                Color32::GRAY,
            );
        }

        if response.drag_started() {
            self.last_drag = None;
        }

        if response.dragged()
            && let Some(mouse) = response.interact_pointer_pos()
        {
            let x = ((mouse.x - plot.left()) / plot.width()).clamp(0.0, 1.0);

            let index = (x * (SAMPLES - 1) as f32).round() as usize;

            let index = index.min(SAMPLES - 1);

            let value = value_from_y(plot, mouse.y).clamp(-1.0, 1.0);

            // Interpolate between mouse events to avoid gaps.
            if let Some((previous_index, previous_value)) = self.last_drag {
                let start = previous_index.min(index);
                let end = previous_index.max(index);

                for i in start..=end {
                    let k = if previous_index == index {
                        1.0
                    } else {
                        (i as f32 - previous_index as f32) / (index as f32 - previous_index as f32)
                    };

                    self.signal[i] = previous_value * (1.0 - k) + value * k;
                }
            } else {
                self.signal[index] = value;
            }

            self.last_drag = Some((index, value));
        }

        if response.drag_stopped() {
            self.last_drag = None;
        }

        for i in 0..SAMPLES - 1 {
            let t0 = i as f32 / (SAMPLES - 1) as f32;
            let t1 = (i + 1) as f32 / (SAMPLES - 1) as f32;

            let p0 = point(plot, t0, self.signal[i]);
            let p1 = point(plot, t1, self.signal[i + 1]);

            painter.line_segment([p0, p1], Stroke::new(2.0, color_at((t0 + t1) * 0.5)));
        }

        // Draw sample points
        for i in 0..SAMPLES {
            let t = i as f32 / (SAMPLES - 1) as f32;

            painter.circle_filled(point(plot, t, self.signal[i]), 1.5, color_at(t));
        }

        // ========================================================
        // SECOND GRAPH: EYE DIAGRAM
        // ========================================================

        ui.add_space(15.0);

        ui.heading("Eye diagram — 4 periods folded into 1");

        let width = ui.available_width().min(500.0);

        let (response, painter) = ui.allocate_painter(Vec2::new(width, 270.0), Sense::hover());

        let rect = response.rect;

        let plot = Rect::from_min_max(
            rect.min + Vec2::new(42.0, 10.0),
            rect.max - Vec2::new(12.0, 27.0),
        );

        draw_grid(&painter, plot, 10, &[("0", 0.0), ("d", 1.0)]);

        for period in 0..PERIODS {
            for j in 0..SAMPLES_PER_PERIOD {
                let index = period * SAMPLES_PER_PERIOD + j;

                // Safety check
                if index + 1 >= self.signal.len() {
                    continue;
                }

                let phase0 = j as f32 / SAMPLES_PER_PERIOD as f32;

                let phase1 = (j + 1) as f32 / SAMPLES_PER_PERIOD as f32;

                let p0 = point(plot, phase0, self.signal[index]);

                let p1 = point(plot, phase1, self.signal[index + 1]);

                // Original time, NOT folded time.
                let original_t0 = index as f32 / (SAMPLES - 1) as f32;

                let original_t1 = (index + 1) as f32 / (SAMPLES - 1) as f32;

                let color = color_at((original_t0 + original_t1) * 0.5);

                painter.line_segment([p0, p1], Stroke::new(2.0, color));
            }
        }
    }
}
