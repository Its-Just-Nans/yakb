//! Coordinates app

use bladvak::{
    ErrorManager,
    eframe::egui::{self, ScrollArea},
};

/// Vector app
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct CoordConverterApp {
    // Decimal Degrees
    dd_lat: f64,
    dd_lon: f64,

    // Degrees Minutes Seconds
    dms_lat_deg: f64,
    dms_lat_min: f64,
    dms_lat_sec: f64,

    dms_lon_deg: f64,
    dms_lon_min: f64,
    dms_lon_sec: f64,

    // Degrees Decimal Minutes
    dmm_lat_deg: f64,
    dmm_lat_min: f64,

    dmm_lon_deg: f64,
    dmm_lon_min: f64,

    // Calculation explanations
    dms_lat_calc: String,
    dms_lon_calc: String,
    dmm_lat_calc: String,
    dmm_lon_calc: String,
}

impl Default for CoordConverterApp {
    fn default() -> Self {
        let mut app = Self {
            dd_lat: 43.409_855,
            dd_lon: 5.230_609,

            dms_lat_deg: 0.0,
            dms_lat_min: 0.0,
            dms_lat_sec: 0.0,

            dms_lon_deg: 0.0,
            dms_lon_min: 0.0,
            dms_lon_sec: 0.0,

            dmm_lat_deg: 0.0,
            dmm_lat_min: 0.0,

            dmm_lon_deg: 0.0,
            dmm_lon_min: 0.0,

            dms_lat_calc: String::new(),
            dms_lon_calc: String::new(),
            dmm_lat_calc: String::new(),
            dmm_lon_calc: String::new(),
        };

        app.calculate_from_dd();
        app
    }
}

impl CoordConverterApp {
    // ---------- CORE CONVERSIONS ----------

    fn calculate_from_dd(&mut self) {
        let lat = self.dd_lat;
        let lon = self.dd_lon;

        self.fill_dms(lat, true);
        self.fill_dms(lon, false);

        self.fill_dmm(lat, true);
        self.fill_dmm(lon, false);
    }

    fn calculate_from_dms(&mut self) {
        let lat = Self::dms_to_dd(self.dms_lat_deg, self.dms_lat_min, self.dms_lat_sec);

        let lon = Self::dms_to_dd(self.dms_lon_deg, self.dms_lon_min, self.dms_lon_sec);

        self.dd_lat = lat;
        self.dd_lon = lon;

        self.fill_dmm(lat, true);
        self.fill_dmm(lon, false);

        self.dms_lat_calc = format!(
            "{} + {}/60 + {}/3600 = {:.6}",
            self.dms_lat_deg, self.dms_lat_min, self.dms_lat_sec, lat
        );

        self.dms_lon_calc = format!(
            "{} + {}/60 + {}/3600 = {:.6}",
            self.dms_lon_deg, self.dms_lon_min, self.dms_lon_sec, lon
        );
    }

    fn calculate_from_dmm(&mut self) {
        let lat = Self::dmm_to_dd(self.dmm_lat_deg, self.dmm_lat_min);

        let lon = Self::dmm_to_dd(self.dmm_lon_deg, self.dmm_lon_min);

        self.dd_lat = lat;
        self.dd_lon = lon;

        self.fill_dms(lat, true);
        self.fill_dms(lon, false);

        self.dmm_lat_calc = format!(
            "{} + {}/60 = {:.6}",
            self.dmm_lat_deg, self.dmm_lat_min, lat
        );

        self.dmm_lon_calc = format!(
            "{} + {}/60 = {:.6}",
            self.dmm_lon_deg, self.dmm_lon_min, lon
        );
    }

    // ---------- HELPERS ----------

    fn fill_dms(&mut self, value: f64, latitude: bool) {
        let deg = value.trunc();
        let min_float = (value - deg).abs() * 60.0;
        let min = min_float.trunc();
        let sec = (min_float - min) * 60.0;

        let calc = format!("({value} - {deg}) × 60 ➡️ {min}' {sec:.3}\"");

        if latitude {
            self.dms_lat_deg = deg;
            self.dms_lat_min = min;
            self.dms_lat_sec = sec;
            self.dms_lat_calc = calc;
        } else {
            self.dms_lon_deg = deg;
            self.dms_lon_min = min;
            self.dms_lon_sec = sec;
            self.dms_lon_calc = calc;
        }
    }

    fn fill_dmm(&mut self, value: f64, latitude: bool) {
        let deg = value.trunc();
        let min = (value - deg).abs() * 60.0;

        let calc = format!("({value} - {deg}) × 60 = {min:.3}'");

        if latitude {
            self.dmm_lat_deg = deg;
            self.dmm_lat_min = min;
            self.dmm_lat_calc = calc;
        } else {
            self.dmm_lon_deg = deg;
            self.dmm_lon_min = min;
            self.dmm_lon_calc = calc;
        }
    }

    fn dms_to_dd(d: f64, m: f64, s: f64) -> f64 {
        (d.abs() + m / 60.0 + s / 3600.0) * if d < 0.0 { -1.0 } else { 1.0 }
    }

    fn dmm_to_dd(d: f64, m: f64) -> f64 {
        (d.abs() + m / 60.0) * if d < 0.0 { -1.0 } else { 1.0 }
    }

    // ---------- UI HELPERS ----------

    fn number_input(ui: &mut egui::Ui, value: &mut f64) {
        ui.add(egui::DragValue::new(value).speed(0.001).max_decimals(6));
    }

    fn calc_label(ui: &mut egui::Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(11.0)
                .color(egui::Color32::GRAY),
        );
    }

    fn section_title(ui: &mut egui::Ui, title: &str) {
        ui.add_space(12.0);
        ui.heading(title);
        ui.separator();
    }
}

impl CoordConverterApp {
    /// show the animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui, _error_manager: &mut ErrorManager) {
        egui::CentralPanel::default().show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(ui.available_width());

                ui.heading("Coordinate Converter");

                // ---------- DD ----------

                Self::section_title(ui, "DD (Decimal Degrees)");

                ui.horizontal(|ui| {
                    ui.label("Latitude");
                    Self::number_input(ui, &mut self.dd_lat);
                });

                ui.horizontal(|ui| {
                    ui.label("Longitude");
                    Self::number_input(ui, &mut self.dd_lon);
                });

                if ui.button("Convert from DD").clicked() {
                    self.calculate_from_dd();
                }

                // ---------- DMS ----------

                Self::section_title(ui, "DMS (Degrees Minutes Seconds)");

                ui.horizontal(|ui| {
                    ui.label("Latitude");

                    Self::number_input(ui, &mut self.dms_lat_deg);
                    ui.label("°");

                    Self::number_input(ui, &mut self.dms_lat_min);
                    ui.label("'");

                    Self::number_input(ui, &mut self.dms_lat_sec);
                    ui.label("\"");
                });

                Self::calc_label(ui, &self.dms_lat_calc);

                ui.horizontal(|ui| {
                    ui.label("Longitude");

                    Self::number_input(ui, &mut self.dms_lon_deg);
                    ui.label("°");

                    Self::number_input(ui, &mut self.dms_lon_min);
                    ui.label("'");

                    Self::number_input(ui, &mut self.dms_lon_sec);
                    ui.label("\"");
                });

                Self::calc_label(ui, &self.dms_lon_calc);

                if ui.button("Convert from DMS").clicked() {
                    self.calculate_from_dms();
                }

                // ---------- DMM ----------

                Self::section_title(ui, "DMM (Degrees Decimal Minutes)");

                ui.horizontal(|ui| {
                    ui.label("Latitude");

                    Self::number_input(ui, &mut self.dmm_lat_deg);
                    ui.label("°");

                    Self::number_input(ui, &mut self.dmm_lat_min);
                    ui.label("'");
                });

                Self::calc_label(ui, &self.dmm_lat_calc);

                ui.horizontal(|ui| {
                    ui.label("Longitude");

                    Self::number_input(ui, &mut self.dmm_lon_deg);
                    ui.label("°");

                    Self::number_input(ui, &mut self.dmm_lon_min);
                    ui.label("'");
                });

                Self::calc_label(ui, &self.dmm_lon_calc);

                if ui.button("Convert from DMM").clicked() {
                    self.calculate_from_dmm();
                }

                ui.add_space(15.0);

                ui.label(
                    egui::RichText::new("* World Geodetic System 84 (WGS 84)")
                        .size(11.0)
                        .color(egui::Color32::GRAY),
                );
            });
        });
    }
}
