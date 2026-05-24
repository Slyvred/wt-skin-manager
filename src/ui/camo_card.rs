use std::time::Duration;

use crate::api::structures::*;
use crate::backend::config::*;
use crate::backend::installer::*;
use crate::components::button::*;
use crate::components::card::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn CamoCard(skin_signal: ReadSignal<Skin>) -> Element {
    let skin = skin_signal.read();

    let user_config = use_context::<Signal<Config>>();

    // Notification on skin installation
    let toast = use_toast();

    rsx! {
        div {
            key: "{skin.file.name}{skin.file.size}",
            style: "display: inline-block; width: 100%; break-inside: avoid; margin-bottom: 1.5rem;",

            Card {
                style: "width: 100%; display: flex; flex-direction: column; overflow: hidden;",

                CardHeader {
                    CardTitle { "Author: {skin.author.nickname}" }
                    CardDescription {
                        div { style: "display: flex; flex-direction: row; align-items: center; gap: 0.8rem; font-size: 0.85rem;",
                            div { style: "display: flex; align-items: center; gap: 0.2rem;", " {skin.likes}" }
                            div { style: "display: flex; align-items: center; gap: 0.2rem;", " {skin.views}" }
                            div { style: "display: flex; align-items: center; gap: 0.2rem;", " {skin.downloads}" }
                            div { style: "display: flex; align-items: center; gap: 0.2rem;", " {skin.comments}" }
                        }
                    }
                }

                CardContent {
                    style: "padding: 10px; display: flex; justify-content: center; align-items: center;",

                    img {
                        src: "{skin.get_thumbnail()}",
                        style: "display: block; max-width: 90%; height: auto; border-radius: 6px;"
                    }
                }

                CardFooter {
                    Button {
                        variant: ButtonVariant::Secondary,
                        style: "width: 100%; margin: 0 auto;",
                        onclick: move |_| {
                            toast.info(
                                "Information".to_string(),
                                ToastOptions::new()
                                    .description(format!("Downloading skin..."))
                                    .duration(Duration::from_secs(5))
                                    .permanent(false)
                            );

                            let skin_copy = skin_signal.read().clone();

                            spawn(async move {
                                match install_skin(skin_copy, user_config).await {
                                    Ok(msg) => {
                                        toast.success(
                                            "Success".to_string(),
                                            ToastOptions::new()
                                                .description(msg)
                                                .duration(Duration::from_secs(5))
                                                .permanent(false)
                                        );
                                    }
                                    Err(err) => {
                                        toast.error(
                                            "Error".to_string(),
                                            ToastOptions::new()
                                                .description(err)
                                                .duration(Duration::from_secs(5))
                                                .permanent(false)
                                        );
                                    }
                                }
                            });
                        },
                        "Install Skin ({(skin.file.size as f32 / 1_000_000.0).round()} MB)"
                    }
                }
            }
        }
    }
}
