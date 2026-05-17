use dioxus::prelude::*;

mod api;
use crate::api::{fetch_filters, fetch_page, fetch_skin, Page, Skin};
use serde_json::{json, Value};

mod components;
use crate::components::button::*;
use crate::components::card::*;
use crate::components::combobox::*;
use crate::components::input::*;
use crate::components::label::*;
use crate::components::pagination::*;
use crate::components::separator::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const ASSETS_CSS: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: ASSETS_CSS }
        // MainPanel {}
        Store {  }
    }
}

#[component]
pub fn Store() -> Element {
    let mut country_query = use_signal(String::new);
    let mut country_value = use_signal(|| None::<String>);

    let mut vehicle_type_query = use_signal(String::new);
    let mut vehicle_type_value = use_signal(|| None::<String>);

    let mut page = use_signal(|| Page::default());
    let mut error_message = use_signal(|| String::new());

    let mut filters = use_signal(|| serde_json::json!({}));
    let mut active_page = use_signal(|| 0);

    let search_action = move |target_page: i32| {
        spawn(async move {
            match fetch_page(
                country_value.read().as_deref().unwrap_or_default(),
                vehicle_type_value.read().as_deref().unwrap_or_default(),
                target_page,
            )
            .await
            {
                Ok(fetched_page) => {
                    page.set(fetched_page);
                }
                Err(err) => {
                    error_message.set(format!("Erreur : {err}"));
                }
            }
        });
    };

    use_hook(move || {
        spawn(async move {
            match fetch_filters().await {
                Ok(fetched_filters) => {
                    filters.set(fetched_filters);
                    error_message.set(String::new());
                }
                Err(err) => {
                    error_message.set(format!("Error : {err}"));
                }
            }
        });
    });

    rsx! {
        div {
            style: "position: fixed; top: 1.25rem",
            Combobox::<String> {
                value: Some(country_value.into()),
                on_value_change: move |next: Option<String>| {
                    country_value.set(next);
                },
                query: Some(country_query()),
                on_query_change: move |next| country_query.set(next),
                placeholder: "Country",
                aria_label: "Country",
                list_aria_label: "Countries",
                ComboboxEmpty { "No country found." }

                if let Some(variants) = filters.read()["vehicleCountry"]["variants"].as_array() {
                    { variants.iter().enumerate().map(|(i, country)| {
                        let value = country["value"].as_str().unwrap_or_default();
                        let name = country["name"].as_str().unwrap_or_default();
                        let count = country["count"].as_i64().unwrap_or_default();
                        let label = format!("{} ({})", name, count);

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{value}",
                                index: i,
                                value: value.to_string(),
                                text_value: name.to_string(),
                                "{label}"
                            }
                        }
                    })}
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(vehicle_type_value.into()),
                on_value_change: move |next: Option<String>| {
                    vehicle_type_value.set(next);
                },
                query: Some(vehicle_type_query()),
                on_query_change: move |next| vehicle_type_query.set(next),
                placeholder: "Type",
                aria_label: "Type",
                list_aria_label: "Types",
                ComboboxEmpty { "No type found." }

                if let Some(variants) = filters.read()["vehicleType"]["variants"].as_array() {
                    { variants.iter().enumerate().map(|(i, vehicule_type)| {
                        let value = vehicule_type["value"].as_str().unwrap_or_default();
                        let name = vehicule_type["name"].as_str().unwrap_or_default();
                        let count = vehicule_type["count"].as_i64().unwrap_or_default();
                        let label = format!("{} ({})", name, count);

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{value}",
                                index: i,
                                value: value.to_string(),
                                text_value: name.to_string(),
                                "{label}"
                            }
                        }
                    })}
                }
            }

            Button {
                style: "margin-left: 0.5rem;",
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    // Quand on fait une nouvelle recherche, on repart de la page 0
                    active_page.set(0);
                    search_action(0);
                },
                "Search",
            }
        }

        if !error_message.read().is_empty() {
            p { style: "color: red;", "{error_message}" }
        }

        if *page.read() != Page::default() {
            ShowPage { page: page }
        }

        Pagination {
            style: "position: fixed; bottom: 1.25rem; margin: 0 auto;",
            PaginationContent {
                style: "background-color: var(--background); border-radius: 10px; padding: 0.3rem",
                PaginationItem {
                    PaginationPrevious {
                        onclick: move |_| {
                            let current = *active_page.read();
                            if current > 0 {
                                let next_p = current - 1;
                                active_page.set(next_p);
                                search_action(next_p); // On relance la recherche avec la page précédente
                            }
                        }
                    }
                }
                PaginationItem {
                    PaginationLink { is_active: true, "{active_page.read()}"}
                }
                PaginationItem {
                    PaginationNext {
                        onclick: move |_| {
                            let next_p = *active_page.read() + 1;
                            active_page.set(next_p);
                            search_action(next_p); // On relance la recherche avec la page suivante
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ShowPage(page: Signal<Page>) -> Element {
    rsx! {
        div { style: "columns: 3 280px; gap: 1.5rem; padding: 25px; width: 100%; max-width: 98vw; margin: 0 auto;",

            { page.read().data.list.iter().map(|skin| rsx! {

                div {
                    key: "{skin.file.name}{skin.file.size}",
                    style: "display: inline-block; width: 100%; break-inside: avoid; margin-bottom: 1.5rem;",

                    Card {
                        style: "width: 100%; display: flex; flex-direction: column; overflow: hidden; margin: 0;",

                        CardHeader {
                            CardTitle { "Author : {skin.author.nickname}" }
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
                                "Install Skin ({(skin.file.size as f32 / 1_000_000.0).round()} MB)"
                            }
                        }
                    }
                }
            })}
        }
    }
}

// #[component]
// pub fn MainPanel() -> Element {
//     let mut skin = use_signal(|| Skin::default());
//     let mut skin_url = use_signal(|| String::new());
//     let mut error_message = use_signal(|| String::new());
//     rsx! {
//         h1 { "War Thunder Skin Manager" }

//         Card { style: "margin: 0 auto; width: 100%; max-width: 80vw;",
//             CardContent {
//                 div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
//                     div { style: "display: grid; gap: 0.5rem;",
//                         Label { html_for: "skin-url", "Skin Url" }
//                         Input {
//                             name: "skin-url",
//                             placeholder: "https://live.warthunder.com/post/1163567/en/",
//                             value: "{skin_url}",
//                             oninput: move |e: FormEvent| skin_url.set(e.value()),
//                         }
//                     }
//                 }
//             }
//             CardFooter { style: "flex-direction: column; gap: 0.5rem;",
//                 Button {
//                     variant: ButtonVariant::Primary,
//                     onclick: move |_| {
//                         let url = skin_url.read().clone();

//                         spawn(async move {
//                             match fetch_skin(&url).await {
//                                 Ok(fetched_skin) => {
//                                     skin.set(fetched_skin);
//                                     error_message.set(String::new());
//                                 }
//                                 Err(err) => {
//                                     error_message.set(format!("Erreur : {err}"));
//                                 }
//                             }
//                         });
//                     },
//                     "Fetch"
//                 }
//             }
//         }

//         if !error_message.read().is_empty() {
//             p { style: "color: red;", "{error_message}" }
//         }

//         if *skin.read() != Skin::default() {
//             SkinCard { fetched_skin: skin }
//             SkinCard { fetched_skin: skin }
//         }
//     }
// }

// #[component]
// pub fn SkinCard(fetched_skin: Signal<Skin>) -> Element {
//     rsx! {
//         Separator {
//             style: "margin: 25px auto; width: 50%;",
//             horizontal: true,
//             decorative: true,
//         }

//         Card {  style: "margin: 0 auto; width: 100%; max-width: 80vw;",
//             // CardHeader {
//             //     CardTitle { "{fetched_skin.read().title}"  }
//             // }

//             CardContent {
//                 div { style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; gap: 1.5rem; width: 100%;",

//                     div { style: "display: grid; gap: 0.5rem;",
//                         ul {
//                             li { "Author : {fetched_skin.read().author.nickname}" }
//                             li { "Likes : {fetched_skin.read().likes}" }
//                             li { "Views: {fetched_skin.read().views}" }
//                             li { "Downloads : {fetched_skin.read().downloads}" }
//                             li { "Comments: {fetched_skin.read().comments}" }

//                             li {
//                                 "Download:"
//                                 ul {
//                                     li { "Filename: {fetched_skin.read().file.name}" }
//                                         li {"Size: {(fetched_skin.read().file.size as f32 / 1e6 as f32).round()} MB" }

//                                 }
//                             }
//                         }
//                     }
//                     img {
//                         src: "{fetched_skin.read().get_thumbnail()}",
//                         style: "max-width: 35%; height: auto; border-radius: 6px;"
//                     }
//                 }
//             }

//             CardFooter { style: "flex-direction: column; gap: 0.5rem;",
//                 Button { variant: ButtonVariant::Secondary, "Install Skin ({(fetched_skin.read().file.size as f32 / 1e6 as f32).round()} MB)" }
//             }

//         }
//     }
// }
