//! Yakb app

use bladvak::{BladvakApp, eframe, eframe::egui};

use crate::swr_app::WaveApp;

/// Yakb app
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct YakbApp {
    /// swr app
    swr: WaveApp,
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
        self.swr.show(ui);
    }
}
