use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MainPanel {}
        // Hero {}
    }
}

fn fetch_skin(url: &str) {}

#[component]
pub fn MainPanel() -> Element {
    let author = "Slyvred";
    rsx! {
        h1 { "War Thunder Skin Manager" }
        h4 { "Brought to you by {author}" }

        input { name: "skin-url", class: "input-url", placeholder: "Skin url (ex: https://live.warthunder.com/post/1163567/en/)" }
        button { class: "btn", "Fetch" }

        img { id: "skin-img", display: "block", src: "https://7a574a09-ed16-41c7-9dc3-b60d230dc7b5.mdnplay.dev/shared-assets/images/examples/firefox-logo.svg", max_height: "20vh", max_width: "auto"}
        p { "Description" }
        button { onclick: fetch_skin(), class: "btn", "Install !"}
    }
}
