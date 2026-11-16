use bladvak::{
    ErrorManager,
    eframe::egui::{self, ScrollArea},
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MotionCalculatorApp {
    // Input values
    hours: u32,
    minutes: u32,
    seconds: u32,
    distance: f64,
    speed: f64,

    last_changed: Option<usize>,

    // History
    rows: Vec<(f64, f64, f64, f64, bool)>,
}

impl Default for MotionCalculatorApp {
    fn default() -> Self {
        let mut app = Self {
            hours: 0,
            minutes: 5,
            seconds: 0,
            distance: 1.0,
            speed: 12.0,
            rows: Vec::new(),
            last_changed: None,
        };

        // Default pace/speed reference values
        for speed in [
            1.0, 4.0, 8.0, 10.0, 12.0, // 5:00 min/km
            15.0, // 4:00 min/km
            20.0, // 3:00 min/km
            30.0, // 2:00 min/km
            60.0, // 1:00 min/km
            90.0, 110.0, 130.0, 150.0, 180.0, 200.0, 250.0,
        ] {
            let pace = 60.0 / speed;
            let time = 3600.0 / speed;

            app.rows.push((
                time,  // Time for 1 km at this speed
                1.0,   // Distance
                pace,  // Pace in min/km
                speed, // Speed in km/h
                true,
            ));
        }

        app
    }
}
impl MotionCalculatorApp {
    /// show the calculator
    pub(crate) fn show(&mut self, ui: &mut egui::Ui, _error_manager: &mut ErrorManager) {
        egui::CentralPanel::default().show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.ui(ui);
            });
        });
    }

    /// ui fn
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::{Color32, DragValue, Grid};

        ui.heading("Pace / Speed Calculator");
        ui.add_space(8.0);

        let mut changed = None;

        // TIME
        ui.horizontal(|ui| {
            ui.label("Time:");

            if ui
                .add(DragValue::new(&mut self.hours).range(0..=999).suffix(" h"))
                .changed()
            {
                changed = Some(0);
            }

            if ui
                .add(
                    DragValue::new(&mut self.minutes)
                        .range(0..=59)
                        .suffix(" min"),
                )
                .changed()
            {
                changed = Some(0);
            }

            if ui
                .add(
                    DragValue::new(&mut self.seconds)
                        .range(0..=59)
                        .suffix(" sec"),
                )
                .changed()
            {
                changed = Some(0);
            }
        });

        // DISTANCE
        ui.horizontal(|ui| {
            ui.label("Distance:");

            if ui
                .add(
                    DragValue::new(&mut self.distance)
                        .speed(0.1)
                        .range(0.0..=100_000.0)
                        .suffix(" km"),
                )
                .changed()
            {
                changed = Some(1);
            }
        });

        // SPEED + READ-ONLY PACE
        ui.horizontal(|ui| {
            ui.label("Speed:");

            if ui
                .add(
                    DragValue::new(&mut self.speed)
                        .speed(0.1)
                        .range(0.0..=1000.0)
                        .suffix(" km/h"),
                )
                .changed()
            {
                changed = Some(2);
            }

            let pace = if self.speed > 0.0 {
                60.0 / self.speed
            } else {
                0.0
            };

            let total_pace_seconds = (pace * 60.0).round() as u32;
            let pace_minutes = total_pace_seconds / 60;
            let pace_seconds = total_pace_seconds % 60;

            if pace_minutes == 0 {
                ui.label(format!("Pace: {pace_seconds:02} s/km"));
            } else {
                ui.label(format!("Pace: {pace_minutes:02}.{pace_seconds:02} min/km"));
            }
        });
        println!("{:?} {:?}", self.last_changed, changed);

        // AUTOMATIC CALCULATION
        if let Some(field) = changed {
            let previous = self.last_changed;
            self.last_changed = Some(field);

            if let Some(previous) = previous {
                let time = f64::from(self.hours) * 3600.0
                    + f64::from(self.minutes) * 60.0
                    + f64::from(self.seconds);

                match (previous.min(field), previous.max(field)) {
                    // Time + Distance => Speed
                    (0, 1) if time > 0.0 && self.distance > 0.0 => {
                        self.speed = self.distance * 3600.0 / time;
                    }

                    // Time + Speed => Distance
                    (0, 2) if time > 0.0 && self.speed > 0.0 => {
                        self.distance = time * self.speed / 3600.0;
                    }

                    // Distance + Speed => Time
                    (1, 2) if self.distance > 0.0 && self.speed > 0.0 => {
                        let total_seconds = (self.distance / self.speed * 3600.0).round() as u64;

                        self.hours = (total_seconds / 3600) as u32;
                        self.minutes = ((total_seconds % 3600) / 60) as u32;
                        self.seconds = (total_seconds % 60) as u32;
                    }

                    _ => {}
                }
            }
        }

        ui.add_space(8.0);

        // INSERT
        if ui.button("Insert").clicked() {
            let time = f64::from(self.hours) * 3600.0
                + f64::from(self.minutes) * 60.0
                + f64::from(self.seconds);

            if time > 0.0 && self.distance > 0.0 && self.speed > 0.0 {
                // Pace in minutes per kilometre
                let pace = 60.0 / self.speed;

                self.rows
                    .push((time, self.distance, pace, self.speed, false));

                // Sort by pace (fastest first)
                self.rows
                    .sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
            }
        }

        ui.separator();
        ui.heading("Results");

        // RESULTS TABLE
        Grid::new("results_table")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.strong("Time");
                ui.strong("Distance (km)");
                ui.strong("Pace (min/km)");
                ui.strong("Speed (km/h)");
                ui.end_row();

                let mut remove = None;

                for (index, (time, distance, pace, speed, is_default)) in
                    self.rows.iter().enumerate()
                {
                    let color = if *is_default {
                        Color32::from_rgb(100, 180, 255)
                    } else {
                        ui.visuals().text_color()
                    };

                    let total_seconds = time.round() as u64;
                    let hours = total_seconds / 3600;
                    let minutes = (total_seconds % 3600) / 60;
                    let seconds = total_seconds % 60;

                    ui.colored_label(color, format!("{hours:02}:{minutes:02}:{seconds:02}"));

                    ui.colored_label(color, format!("{distance:.3}"));

                    let pace_seconds = (pace * 60.0).round() as u32;
                    ui.colored_label(
                        color,
                        format!("{}:{:02}", pace_seconds / 60, pace_seconds % 60),
                    );

                    ui.colored_label(color, format!("{speed:.2}"));

                    if ui.small_button("✕").clicked() {
                        remove = Some(index);
                    }

                    ui.end_row();
                }

                if let Some(index) = remove {
                    self.rows.remove(index);
                }
            });

        ui.add_space(8.0);

        // RESET
        if ui.button("Reset to defaults").clicked() {
            *self = Self::default();
        }
    }
}
