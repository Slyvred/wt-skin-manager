use std::time::Duration;

use crate::api::structures::*;
use crate::backend::config::*;
use crate::backend::uninstaller::*;
use crate::components::button::*;
use crate::components::card::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions, Toasts};

fn uninstall(toast: Toasts, user_config: Signal<Config>, skin_signal: ReadSignal<Skin>) {
    toast.info(
        "Information".to_string(),
        ToastOptions::new()
            .description(format!("Downloading skin..."))
            .duration(Duration::from_secs(5))
            .permanent(false),
    );

    spawn(async move {
        match uninstall_skin(skin_signal, user_config).await {
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
pub fn CamoCardUninstall(skin_signal: ReadSignal<Skin>) -> Element {
    let skin = skin_signal.read();

    let user_config = use_context::<Signal<Config>>();

    // Notification on skin installation
    let toast = use_toast();

    rsx! {
        div {
            key: "{skin.file.name}{skin.file.size}",
            style: "display: inline-block; width: 100%; break-inside: avoid; margin-bottom: 1.5rem;",

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
                        variant: ButtonVariant::Destructive,
                        class: "w-full mx-auto",
                        onclick: move |_| { uninstall(toast, user_config, skin_signal); },
                        "Uninstall Skin"
                    }
                }
            }
        }
    }
}
