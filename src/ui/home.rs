use crate::backend::config::*;
use crate::components::sidebar::*;
use crate::components::toast::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;

use crate::ui::camo_page::CamoPage;
use crate::ui::config_modal::ConfigModal;
use crate::ui::uninstall_page::UninstallPage;

#[component]
pub fn Home() -> Element {
    let mut user_config = use_context::<Signal<Config>>();

    let confirmed = use_signal(|| false);

    let mut show_feed = use_signal(|| true);
    let mut show_uninstall = use_signal(|| false);

    let mut open = use_signal(|| false);

    match Config::load_from_file() {
        Ok(config) => {
            user_config.set(config);
        }
        Err(e) => {
            tracing::debug!("Failed to load config, opening modal: {:?}", e);
            open.set(true);
        }
    }

    rsx! {
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
                            rsx! { CamoPage {} }
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
