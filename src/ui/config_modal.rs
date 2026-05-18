use crate::backend::config::Config;
use crate::components::alert_dialog::*;
use crate::components::button::*;
use crate::components::input::*;
use crate::components::label::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::path::Path;
use std::time::Duration;

#[component]
pub fn ConfigModal(open: Signal<bool>, confirmed: Signal<bool>) -> Element {
    let mut user_config = use_context::<Signal<Result<Config, String>>>();
    let mut game_dir = use_context::<Signal<String>>();
    let config_toast = use_toast();

    rsx! {
        AlertDialog { open: open(), on_open_change: move |v| open.set(v),
            AlertDialogTitle { "No config detected !" }
            AlertDialogDescription { "Please set your game installation's folder. This is where your camouflages will be downloaded and extracted." }
            div { display: "flex", flex_direction: "column", gap: ".5rem",
                Label { html_for: "gamedir", "Path" }
                Input {
                    id: "gamedir",
                    oninput: move |e: FormEvent| game_dir.set(e.value()),
                    placeholder: "Ex: /home/user/Games/WarThunder/UserSkins"
                }
            }
            AlertDialogActions {
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        let game_dir = game_dir.read().clone();
                        let path = Path::new(&game_dir);

                        if path.exists() {
                            let cfg = Config { version: "1.0.0".to_string(), game_dir: game_dir};
                            let res = cfg.save();

                            if res.is_err() {
                                let _ = dbg!("{:?}", &res);

                                config_toast.error(
                                    "Error".to_string(),
                                    ToastOptions::new()
                                        .description(format!("Failed to save config: {}", res.unwrap_err()))
                                        .duration(Duration::from_secs(3))
                                        .permanent(false)
                                );
                            }

                            user_config.set(Ok(cfg));

                            config_toast.success(
                                "Success".to_string(),
                                ToastOptions::new()
                                    .description("Saved path !".to_string())
                                    .duration(Duration::from_secs(3))
                                    .permanent(false)
                            );

                            confirmed.set(true);
                            open.set(false);

                        }
                        else {
                            confirmed.set(false);

                            config_toast.error(
                                "Error".to_string(),
                                ToastOptions::new()
                                    .description("Please set a valid path".to_string())
                                    .duration(Duration::from_secs(3))
                                    .permanent(false)
                            );
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
