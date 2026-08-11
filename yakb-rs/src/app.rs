//! Yakb app

use bladvak::{BladvakApp, eframe, eframe::egui};

use crate::swr_app::WaveApp;

/// All available animations
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub(crate) enum Animation {
    /// base animation
    #[default]
    Base,
    /// SWR animation
    Swr(WaveApp),
}

impl Animation {
    /// Show the current animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        match self {
            Animation::Base => {
                bladvak::utils::central_ui(ui, |ui| {
                    ui.label("Welcome to yakb");
                });
            }
            Animation::Swr(wave_app) => wave_app.show(ui),
        }
    }
}

/// Yakb app
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct YakbApp {
    /// current animation
    animation: Animation,
}

impl BladvakApp<'_> for YakbApp {
    fn try_new_with_args(
        saved_state: Self,
        _cc: &eframe::CreationContext<'_>,
        _args: &[String],
        _error_manager: &mut bladvak::ErrorManager,
    ) -> Result<Self, bladvak::AppError> {
        Ok(saved_state)
    }

    fn name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn repo_url() -> String {
        "https://github.com/Its-Just-Nans/yakb".to_string()
    }

    fn central_panel(&mut self, ui: &mut egui::Ui, _error_manager: &mut bladvak::ErrorManager) {
        self.animation.show(ui);
    }

    fn menu_file(&mut self, ui: &mut egui::Ui, _error_manager: &mut bladvak::ErrorManager) {
        ui.menu_button("Animation", |ui| {
            if ui.button("Base").clicked() {
                self.animation = Animation::Base;
            }
            if ui.button("SWR").clicked() {
                self.animation = Animation::Swr(WaveApp::default());
            }
        });
    }
}
