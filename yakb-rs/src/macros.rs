//! Macro

/// Define the animations and automatically do the boring stuff
macro_rules! define_animations {
    (
        $(
            $(#[$attr:meta])*
            $variant:ident($app:ty) {
                title: $title:literal,
                name: $name:literal,
            }
        ),* $(,)?
    ) => {
        /// All available animations
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub(crate) enum Animation {
            Base,
            $(
                $(#[$attr])*
                $variant($app),
            )*
        }

        impl Default for Animation {
            fn default() -> Self {
                Self::Base
            }
        }

        impl Animation {
            /// Show the current animation
            pub(crate) fn show(
                &mut self,
                ui: &mut egui::Ui,
                error_manager: &mut ErrorManager,
            ) {
                match self {
                    Self::Base => {
                        bladvak::utils::central_ui(ui, |ui| {
                            ui.label("Welcome to yakb");
                        });
                    }
                    $(
                        Self::$variant(app) => {
                            app.show(ui, error_manager);
                        }
                    )*
                }
            }
        }

        impl YakbApp {
            /// show the menu
            pub(crate) fn show_menu(&mut self, ui: &mut egui::Ui) {
                if ui
                    .selectable_label(matches!(self.animation, Animation::Base), "Base")
                    .clicked()
                {
                    self.animation = Animation::Base;
                }
                $(

                    if ui.selectable_label(
                            matches!(self.animation, Animation::$variant(_)),
                            $title,
                        )
                        .clicked()
                    {
                        self.animation = Animation::$variant(<$app>::default());
                    }
                )*
            }
        }

        /// For WASM arguments in the URL
        impl TryFrom<&str> for Animation {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $name => Ok(Self::$variant(<$app>::default())),
                    )*
                    _ => Err("No animation".to_string()),
                }
            }
        }
    };
}

pub(crate) use define_animations;
