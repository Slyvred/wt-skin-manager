#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod api;
mod backend;
mod components;
mod ui;

use crate::components::sidebar::*;
use crate::components::toast::*;
use crate::ui::camo_feed::*;
use crate::ui::config_modal::*;
use crate::ui::uninstall_page::UninstallPage;
use backend::config::Config;
use dioxus::desktop::LogicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use reqwest::Client;

const FAVICON: Asset = asset!("/assets/Imil-Sea-Crab.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const ASSETS_CSS: Asset = asset!("/assets/dx-components-theme.css");

// Contains our custom for the pretty glyphs :)
#[used]
static CASKAYDIA_MONO_NERD_FONT: Asset = asset!(
    "/assets/fonts/CaskaydiaMonoNerdFont-Regular.ttf",
    manganis::AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .with_title("War Thunder Skin Manager");

    let config = dioxus::desktop::Config::new()
        .with_menu(None)
        .with_window(window);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut user_config = use_signal(|| Config::default());
    let client = use_signal(|| Client::new());

    let mut open = use_signal(|| false);
    let confirmed = use_signal(|| false);

    let mut show_feed = use_signal(|| true);
    let mut show_uninstall = use_signal(|| false);

    match Config::load_from_file() {
        Ok(config) => {
            user_config.set(config);
        }
        Err(e) => {
            tracing::debug!("{:?}", e);
            open.set(true);
        }
    }

    tracing::debug!("FONTS PATH: {:?}", CASKAYDIA_MONO_NERD_FONT);

    provide_context(user_config);
    provide_context(client);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: ASSETS_CSS }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        ToastProvider {
            SidebarProvider {
                Sidebar {
                    side: SidebarSide::Left,
                    variant: SidebarVariant::Sidebar,
                    collapsible: SidebarCollapsible::Offcanvas,

                    SidebarHeader {
                        SidebarTrigger {}
                    }

                    SidebarContent {
                        SidebarGroup {
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: *show_feed.read(),
                                            div {
                                                style: "display: block; width: 100%;",
                                                onclick: move |_| {
                                                    show_feed.set(true);
                                                    show_uninstall.set(false);
                                                },
                                                " Install Skins"
                                            }
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: *show_uninstall.read(),
                                            div {
                                                style: "display: block; width: 100%;",
                                                onclick: move |_| {
                                                    show_uninstall.set(true);
                                                    show_feed.set(false);
                                                },
                                                " Uninstall Skins"
                                            }
                                        }
                                    }
                                    // SidebarMenuItem {
                                    //     SidebarMenuButton {
                                    //         is_active: false,
                                    //         span { "Settings" }
                                    //     }
                                    // }
                                }
                            }
                        }
                    }
                }

                SidebarRail {}

                SidebarInset {
                    style: "display: flex; flex-direction: column; flex: 1; height: 100vh; overflow-y: auto;",
                    id: "inset",
                    {
                        if *show_feed.read() {
                            rsx! { CamoFeed {} }
                        } else if *show_uninstall.read() {
                            rsx! { UninstallPage {} }
                        }
                        else {
                            rsx! { span { "Select a tab to continue" } }
                        }
                    }
                    ConfigModal { open, confirmed }
                }
            }
        }
    }
}
