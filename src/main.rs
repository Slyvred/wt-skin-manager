#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod api;
mod backend;
mod components;
mod ui;

use dioxus::desktop::LogicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use reqwest::Client;

use crate::backend::config::Config;
use crate::ui::home::Home;

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
pub fn App() -> Element {
    let user_config = use_signal(|| Config::default());
    let client = use_signal(|| Client::new());
    tracing::debug!("FONTS PATH: {:?}", CASKAYDIA_MONO_NERD_FONT);

    provide_context(user_config);
    provide_context(client);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: ASSETS_CSS }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        Home {  }
    }
}
