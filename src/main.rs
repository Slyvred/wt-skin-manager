#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod api;
mod backend;
mod components;
mod ui;

use crate::components::toast::*;
use crate::ui::camo_feed::*;
use crate::ui::config_modal::*;
use backend::config::Config;
use dioxus::desktop::LogicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;

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
        // .with_transparent(true)
        // .with_decorations(false)
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
    let user_config = use_signal(|| Config::load_from_file("./config.json"));
    // let game_dir = use_signal(String::new);
    let open = use_signal(|| user_config.read().is_err());
    let confirmed = use_signal(|| false);

    let _ = dbg!("FONTS PATH: {:?}", CASKAYDIA_MONO_NERD_FONT);

    provide_context(user_config);
    // provide_context(game_dir);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: ASSETS_CSS }

        ToastProvider {
            ConfigModal { open, confirmed }
            CamoFeed { }
        }
    }
}
