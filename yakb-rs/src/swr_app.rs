///! swr app

use bladvak::eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct WaveApp {
    reflection: f32,
    length: f32,

    incident: bool,
    reflected: bool,
    total: bool,
    stationary: bool,
    progressive: bool,

    time: f32,
}

impl Default for WaveApp {
    fn default() -> Self {
        Self {
            reflection: -0.60,
            length: 3.0,

            incident: true,
            reflected: true,
            total: true,
            stationary: false,
            progressive: true,

            time: 0.0,
        }
    }
}

// ============================================================================
// WAVE CANVAS
// ============================================================================

fn draw_wave_canvas(
    painter: &egui::Painter,
    rect: Rect,
    time: f32,
    reflection: f32,
    length: f32,
    show_incident: bool,
    show_reflected: bool,
    show_total: bool,
    show_stationary: bool,
    show_progressive: bool,
) {
    let plot = rect;

    let axis_y = plot.center().y + 10.0;

    // ------------------------------------------------------------------------
    // Axis
    // ------------------------------------------------------------------------

    painter.line_segment(
        [
            Pos2::new(plot.left(), axis_y),
            Pos2::new(plot.right(), axis_y),
        ],
        Stroke::new(1.0, Color32::LIGHT_GRAY),
    );

    // ------------------------------------------------------------------------
    // Labels
    // ------------------------------------------------------------------------

    painter.text(
        Pos2::new(plot.left() + 6.0, plot.top() + 8.0),
        Align2::LEFT_TOP,
        "Emetteur",
        FontId::proportional(16.0),
        Color32::GRAY,
    );

    painter.text(
        Pos2::new(plot.right() - 80.0, plot.top() + 8.0),
        Align2::LEFT_TOP,
        "Récepteur",
        FontId::proportional(16.0),
        Color32::GRAY,
    );

    // ------------------------------------------------------------------------
    // Parameters
    // ------------------------------------------------------------------------

    // Slider range is 0.25 ..= 3.0.
    //
    // Map it to a useful visual wavelength.
    let wavelength = egui::lerp(80.0..=500.0, length / 3.0);

    let amplitude = 50.0;

    // Angular frequency.
    let omega = 2.0;

    // ------------------------------------------------------------------------
    // Incident wave
    // ------------------------------------------------------------------------

    if show_incident {
        draw_incident_wave(
            painter,
            plot,
            axis_y,
            wavelength,
            amplitude,
            omega,
            time,
            Color32::RED,
        );
    }

    // ------------------------------------------------------------------------
    // Reflected wave
    // ------------------------------------------------------------------------

    if show_reflected {
        draw_reflected_wave(
            painter,
            plot,
            axis_y,
            wavelength,
            amplitude,
            reflection,
            omega,
            time,
            Color32::GREEN,
        );
    }

    // ------------------------------------------------------------------------
    // Total wave
    // ------------------------------------------------------------------------

    if show_total {
        draw_total_wave(
            painter,
            plot,
            axis_y,
            wavelength,
            amplitude,
            reflection,
            omega,
            time,
            Color32::BLACK,
        );
    }

    // ------------------------------------------------------------------------
    // Stationary wave
    // ------------------------------------------------------------------------

    if show_stationary {
        draw_stationary_wave(
            painter,
            plot,
            axis_y,
            wavelength,
            amplitude,
            reflection,
            omega,
            time,
            Color32::BLUE,
        );
    }

    // ------------------------------------------------------------------------
    // Progressive component
    // ------------------------------------------------------------------------

    if show_progressive {
        draw_progressive_wave(
            painter,
            plot,
            axis_y,
            wavelength,
            amplitude,
            reflection,
            omega,
            time,
            Color32::MAGENTA,
        );
    }

    // ------------------------------------------------------------------------
    // Legend
    // ------------------------------------------------------------------------

    let x = plot.right() + 8.0;

    let legend = [
        ("Onde Incidente", Color32::RED),
        ("Onde Réfléchie", Color32::GREEN),
        ("Onde Totale", Color32::BLACK),
        ("Onde Stationnaire", Color32::BLUE),
        ("Composante Progressive", Color32::MAGENTA),
    ];

    let mut y = plot.top() + 20.0;

    for (name, color) in legend {
        painter.line_segment(
            [Pos2::new(x, y + 8.0), Pos2::new(x + 25.0, y + 8.0)],
            Stroke::new(2.0, color),
        );

        painter.text(
            Pos2::new(x + 30.0, y),
            Align2::LEFT_TOP,
            name,
            FontId::proportional(18.0),
            color,
        );

        y += 27.0;
    }
}

// ============================================================================
// INCIDENT WAVE
// ============================================================================

fn draw_incident_wave(
    painter: &egui::Painter,
    rect: Rect,
    axis_y: f32,
    wavelength: f32,
    amplitude: f32,
    omega: f32,
    time: f32,
    color: Color32,
) {
    let mut points = Vec::new();

    let k = std::f32::consts::TAU / wavelength;

    let mut x = rect.left();

    while x <= rect.right() {
        let local_x = x - rect.left();

        // y_i = A sin(kx - ωt)
        let value = (k * local_x - omega * time).sin();

        let y = axis_y - amplitude * value;

        points.push(Pos2::new(x, y));

        x += 2.0;
    }

    painter.add(egui::Shape::line(points, Stroke::new(1.3, color)));
}

// ============================================================================
// REFLECTED WAVE
// ============================================================================

fn draw_reflected_wave(
    painter: &egui::Painter,
    rect: Rect,
    axis_y: f32,
    wavelength: f32,
    amplitude: f32,
    reflection: f32,
    omega: f32,
    time: f32,
    color: Color32,
) {
    let mut points = Vec::new();

    let k = std::f32::consts::TAU / wavelength;

    let mut x = rect.left();

    while x <= rect.right() {
        let local_x = x - rect.left();

        // y_r = R A sin(kx + ωt)
        //
        // The sign of R is preserved.
        //
        // R = +1 -> reflected wave in phase
        // R = -1 -> reflected wave inverted
        let value = reflection * (k * local_x + omega * time).sin();

        let y = axis_y - amplitude * value;

        points.push(Pos2::new(x, y));

        x += 2.0;
    }

    painter.add(egui::Shape::line(points, Stroke::new(1.3, color)));
}

// ============================================================================
// TOTAL WAVE
// ============================================================================

fn draw_total_wave(
    painter: &egui::Painter,
    rect: Rect,
    axis_y: f32,
    wavelength: f32,
    amplitude: f32,
    reflection: f32,
    omega: f32,
    time: f32,
    color: Color32,
) {
    let mut points = Vec::new();

    let k = std::f32::consts::TAU / wavelength;

    let mut x = rect.left();

    while x <= rect.right() {
        let local_x = x - rect.left();

        // Incident:
        //
        // A sin(kx - ωt)
        //
        let incident = (k * local_x - omega * time).sin();

        // Reflected:
        //
        // R A sin(kx + ωt)
        //
        let reflected = reflection * (k * local_x + omega * time).sin();

        // Superposition:
        //
        // y = y_i + y_r
        //
        let total = incident + reflected;

        let y = axis_y - amplitude * total;

        points.push(Pos2::new(x, y));

        x += 2.0;
    }

    painter.add(egui::Shape::line(points, Stroke::new(1.6, color)));
}

// ============================================================================
// STATIONARY WAVE
// ============================================================================

fn draw_stationary_wave(
    painter: &egui::Painter,
    rect: Rect,
    axis_y: f32,
    wavelength: f32,
    amplitude: f32,
    reflection: f32,
    omega: f32,
    time: f32,
    color: Color32,
) {
    let mut points = Vec::new();

    let k = std::f32::consts::TAU / wavelength;

    let r = reflection.abs();

    let mut x = rect.left();

    while x <= rect.right() {
        let local_x = x - rect.left();

        let value = if reflection >= 0.0 {
            // --------------------------------------------------------------
            // R >= 0
            //
            // sin(kx - ωt) + R sin(kx + ωt)
            //
            // For R = 1:
            //
            // 2 sin(kx) cos(ωt)
            // --------------------------------------------------------------

            let spatial = (k * local_x).sin();
            let temporal = (omega * time).cos();

            2.0 * r * spatial * temporal
        } else {
            // --------------------------------------------------------------
            // R < 0
            //
            // For R = -1:
            //
            // sin(kx - ωt) - sin(kx + ωt)
            //
            // = -2 cos(kx) sin(ωt)
            // --------------------------------------------------------------

            let spatial = (k * local_x).cos();
            let temporal = (omega * time).sin();

            -2.0 * r * spatial * temporal
        };

        let y = axis_y - amplitude * value;

        points.push(Pos2::new(x, y));

        x += 2.0;
    }

    painter.add(egui::Shape::line(points, Stroke::new(1.5, color)));
}

// ============================================================================
// PROGRESSIVE COMPONENT
// ============================================================================

fn draw_progressive_wave(
    painter: &egui::Painter,
    rect: Rect,
    axis_y: f32,
    wavelength: f32,
    amplitude: f32,
    reflection: f32,
    omega: f32,
    time: f32,
    color: Color32,
) {
    let mut points = Vec::new();

    let k = std::f32::consts::TAU / wavelength;

    // The travelling component disappears when |R| = 1.
    //
    // R = 0:
    //     full progressive wave
    //
    // |R| = 1:
    //     pure standing wave
    //
    let progressive_amplitude = amplitude * (1.0 - reflection.abs());

    let mut x = rect.left();

    while x <= rect.right() {
        let local_x = x - rect.left();

        // Same direction as the incident wave.
        //
        // sin(kx - ωt)
        let value = (k * local_x - omega * time).sin();

        let y = axis_y - progressive_amplitude * value;

        points.push(Pos2::new(x, y));

        x += 2.0;
    }

    painter.add(egui::Shape::line(points, Stroke::new(1.4, color)));
}

// ============================================================================
// CONTROLS
// ============================================================================

fn draw_controls(ui: &mut egui::Ui, app: &mut WaveApp) {
    let panel_width = 320.0;

    ui.set_width(panel_width);

    ui.separator();

    // ------------------------------------------------------------------------
    // Reflection coefficient
    // ------------------------------------------------------------------------

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Coeff de Réflexion").color(Color32::WHITE));

        ui.add(egui::Slider::new(&mut app.reflection, -1.0..=1.0).show_value(false));

        ui.label(egui::RichText::new(format!("{:.2}", app.reflection)).color(Color32::LIGHT_BLUE));
    });

    // ------------------------------------------------------------------------
    // Waves
    // ------------------------------------------------------------------------

    checkbox(ui, "Onde Incidente", &mut app.incident);

    checkbox(ui, "Onde Réfléchie", &mut app.reflected);

    checkbox(ui, "Onde Totale", &mut app.total);

    checkbox(ui, "Onde Stationnaire", &mut app.stationary);

    checkbox(ui, "Composante Progressive", &mut app.progressive);

    // ------------------------------------------------------------------------
    // Wavelength
    // ------------------------------------------------------------------------

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Longueur").color(Color32::WHITE));

        ui.add(egui::Slider::new(&mut app.length, 0.25..=3.0).show_value(false));

        ui.label(egui::RichText::new(format!("{:.2}", app.length)).color(Color32::LIGHT_BLUE));
    });
}

fn checkbox(ui: &mut egui::Ui, text: &str, value: &mut bool) {
    ui.checkbox(value, egui::RichText::new(text).color(Color32::WHITE));
}

impl WaveApp {
    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();

        // Animation
        self.time += ctx.input(|i| i.stable_dt);

        ctx.request_repaint();

        ui.vertical_centered(|ui| {
            ui.heading(
                egui::RichText::new("Réflexion et Onde Stationnaire")
                    .size(34.0)
                    .strong(),
            );
        });

        ui.add_space(10.0);

        // ====================================================================
        // CANVAS
        // ====================================================================
        ui.horizontal(|ui| {
            let available = ui.available_size();

            let canvas_width = (available.x).max(500.0);

            let canvas_height = 300.0;

            egui::Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.set_min_size(Vec2::new(canvas_width, canvas_height));

                    let rect = ui.max_rect();

                    let painter = ui.painter_at(rect);

                    draw_wave_canvas(
                        &painter,
                        rect,
                        self.time,
                        self.reflection,
                        self.length,
                        self.incident,
                        self.reflected,
                        self.total,
                        self.stationary,
                        self.progressive,
                    );
                });
        });

        ui.add_space(10.0);

        // ====================================================================
        // CONTROLS
        // ====================================================================

        draw_controls(ui, self);

        ui.add_space(10.0);

        // ====================================================================
        // INFORMATION
        // ====================================================================

        egui::Grid::new("coefficient_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Coefficient de Réflexion").size(16.0));

                ui.label(egui::RichText::new(format!("{:.2}", self.reflection)).size(16.0));

                ui.end_row();

                ui.label("R / Z₁");

                ui.label(
                    egui::RichText::new(format!(
                        "{:.6}",
                        (1.0 + self.reflection) / (1.0 - self.reflection)
                    ))
                    .size(16.0),
                );

                ui.end_row();

                ui.label("Longueur d'onde");

                let wavelength = egui::lerp(80.0..=500.0, self.length / 3.0);

                ui.label(egui::RichText::new(format!("{:.1} px", wavelength)).size(16.0));

                ui.end_row();

                ui.label("Temps");

                ui.label(egui::RichText::new(format!("{:.2}", self.time)).size(16.0));

                ui.end_row();
            });
    }
}
