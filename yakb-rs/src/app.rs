//! Yakb app

use bladvak::{
    BladvakApp,
    eframe::{self, egui},
};

use crate::swr_app::WaveApp;
use crate::unit_circle_app::UnitCircleApp;
use crate::vector_app::VectorApp;

/// All available animations
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub(crate) enum Animation {
    /// base animation
    #[default]
    Base,
    /// SWR animation
    Swr(WaveApp),
    /// Vector animation
    Vector(VectorApp),
    /// Unit circle animation
    UnitCircle(UnitCircleApp),
}

impl Animation {
    /// Show the current animation
    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::Base => {
                bladvak::utils::central_ui(ui, |ui| {
                    ui.label("Welcome to yakb");
                });
            }
            Self::Swr(wave_app) => {
                egui::ScrollArea::vertical()
                    .id_salt("animation_viewer")
                    .show(ui, |ui| wave_app.show(ui));
            }
            Self::Vector(app) => app.show(ui),
            Self::UnitCircle(app) => app.show(ui),
        }
    }
}

/// For WASM arg in URL
impl TryFrom<&str> for Animation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "swr" => Ok(Self::Swr(WaveApp::default())),
            "vector" => Ok(Self::Vector(VectorApp::default())),
            "unit_circle" => Ok(Self::UnitCircle(UnitCircleApp::default())),
            _ => Err("No animation".to_string()),
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
        #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))] mut saved_state: Self,
        #[cfg_attr(not(target_arch = "wasm32"), allow(unused))] cc: &eframe::CreationContext<'_>,
        _args: &[String],
        _error_manager: &mut bladvak::ErrorManager,
    ) -> Result<Self, bladvak::AppError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(names) = cc
                .integration_info
                .web_info
                .location
                .query_map
                .get("animation")
                && let Some(name) = names.first()
                && let Ok(anim) = Animation::try_from(name.as_str())
            {
                saved_state.animation = anim;
            }
        }
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
            if ui
                .selectable_label(matches!(self.animation, Animation::Base), "Base")
                .clicked()
            {
                self.animation = Animation::Base;
            }
            if ui
                .selectable_label(matches!(self.animation, Animation::Swr(_)), "SWR")
                .clicked()
            {
                self.animation = Animation::Swr(WaveApp::default());
            }
            if ui
                .selectable_label(matches!(self.animation, Animation::Vector(_)), "Vector")
                .clicked()
            {
                self.animation = Animation::Vector(VectorApp::default());
            }
            if ui
                .selectable_label(
                    matches!(self.animation, Animation::UnitCircle(_)),
                    "Unit circle",
                )
                .clicked()
            {
                self.animation = Animation::UnitCircle(UnitCircleApp::default());
            }
        });
    }
}
