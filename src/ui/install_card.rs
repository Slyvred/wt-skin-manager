use std::time::Duration;

use crate::api::structures::*;
use crate::backend::config::*;
use crate::backend::installer::*;
use crate::components::button::*;
use crate::components::card::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions, Toasts};

fn install(toast: Toasts, user_config: Signal<Config>, skin_signal: ReadSignal<Skin>) {
    toast.info(
        "Information".to_string(),
        ToastOptions::new()
            .description(format!("Downloading skin..."))
            .duration(Duration::from_secs(5))
            .permanent(false),
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
                        .permanent(false),
                );
            }
            Err(err) => {
                toast.error(
                    "Error".to_string(),
                    ToastOptions::new()
                        .description(err)
                        .duration(Duration::from_secs(5))
                        .permanent(false),
                );
            }
        }
    });
}

#[component]
pub fn CamoCard(skin_signal: ReadSignal<Skin>) -> Element {
    let skin = skin_signal.read();

    let user_config = use_context::<Signal<Config>>();

    // Notification on skin installation
    let toast = use_toast();

    rsx! {
        div {
            key: "{skin.file.name}{skin.file.size}",
            class: "inline-block w-full break-inside-avoid mb-6",

            Card {
                class: "w-full flex flex-col overflow-hidden",

                CardHeader {
                    CardTitle { "Author: {skin.author.nickname}" }
                    CardDescription {
                        div { class: "flex flex-row items-center gap-3 text-xs sm:text-sm",
                            div { class: "flex items-center gap-1", " {skin.likes}" }
                            div { class: "flex items-center gap-1", " {skin.views}" }
                            div { class: "flex items-center gap-1", " {skin.downloads}" }
                            div { class: "flex items-center gap-1", " {skin.comments}" }
                        }
                    }
                }

                CardContent {
                    class: "p-2.5 flex justify-center items-center",

                    img {
                        src: "{skin.get_thumbnail()}",
                        class: "block max-w-[90%] h-auto rounded-md"
                    }
                }

                CardFooter {
                    Button {
                        variant: ButtonVariant::Secondary,
                        class: "w-full mx-auto",
                        onclick: move |_| { install(toast, user_config, skin_signal); },
                        "Install Skin ({(skin.file.size as f32 / 1_000_000.0).round()} MB)"
                    }
                }
            }
        }
    }
}
