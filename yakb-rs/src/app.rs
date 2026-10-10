//! Yakb app

use bladvak::{
    BladvakApp, ErrorManager,
    eframe::{self, egui},
};

use crate::macros::define_animations;
use crate::unit_circle_app::UnitCircleApp;
use crate::vector_app::VectorApp;
use crate::{boat_wind_app::BoatWindApp, swr_app::WaveApp};

define_animations! {
    /// SWR animation
    Swr(WaveApp) {
        title: "SWR",
        name: "swr",
    },

    /// Vector animation
    Vector(VectorApp) {
        title: "Vector",
        name: "vector",
    },

    /// Unit circle animation
    UnitCircle(UnitCircleApp) {
        title: "Unit Circle",
        name: "unit_circle",
    },

    /// Boat wind animation
    BoatWind(BoatWindApp) {
        title: "Boat wind",
        name: "boat_wind",
    },
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

    fn central_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.animation.show(ui, error_manager);
    }

    fn menu_file(&mut self, ui: &mut egui::Ui, _error_manager: &mut bladvak::ErrorManager) {
        ui.menu_button("Animation", |ui| {
            self.show_menu(ui);
        });
    }
}
