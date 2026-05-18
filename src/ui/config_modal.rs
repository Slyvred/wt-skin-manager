use crate::backend::config::Config;
use crate::components::alert_dialog::*;
use crate::components::button::*;
use crate::components::input::*;
use crate::components::label::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::path::Path;
use std::time::Duration;

fn save_config(
    mut open: Signal<bool>,
    mut confirmed: Signal<bool>,
    game_dir: Signal<String>,
) -> Result<Config, String> {
    let game_dir = game_dir.read().clone();
    let path = Path::new(&game_dir);

    if !path.exists() {
        confirmed.set(false);
        return Err("Please set a valid path".to_string());
    }

    let config = Config {
        version: "1.0.0".to_string(),
        game_dir: game_dir,
    };
    let res = config.save();

    match res {
        Ok(_) => {
            confirmed.set(true);
            open.set(false);
            Ok(config)
        }
        Err(e) => {
            let _ = dbg!("{:?}", &e);
            Err(format!("Failed to save config: {}", e))
        }
    }
}

#[component]
pub fn ConfigModal(open: Signal<bool>, confirmed: Signal<bool>) -> Element {
    let mut game_dir = use_signal(String::new);
    let mut user_config = use_context::<Signal<Result<Config, String>>>();
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
                        match save_config(open, confirmed, game_dir) {
                            Ok(cfg) => {
                                user_config.set(Ok(cfg));
                                config_toast.success(
                                    "Error".to_string(),
                                    ToastOptions::new()
                                        .description("Saved path !".to_string())
                                        .duration(Duration::from_secs(3))
                                        .permanent(false),
                                );
                            }
                            Err(e) => {
                                config_toast.error(
                                    "Error".to_string(),
                                    ToastOptions::new()
                                        .description(e)
                                        .duration(Duration::from_secs(3))
                                        .permanent(false),
                                );
                            }
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
