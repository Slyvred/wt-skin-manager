use dioxus::prelude::*;

mod api;
use crate::api::{fetch_page, fetch_skin, Page, Skin};

mod components;
use crate::components::button::*;
use crate::components::card::*;
use crate::components::combobox::*;
use crate::components::input::*;
use crate::components::label::*;
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

    let countries: &[(&str, &str)] = &[
        ("", "Any"),
        ("britain", "Great Britain"),
        ("china", "China"),
        ("france", "France"),
        ("germany", "Germany"),
        ("italy", "Italy"),
        ("japan", "Japan"),
        ("south_africa", "South Africa"),
        ("sweden", "Sweden"),
        ("usa", "USA"),
        ("ussr", "USSR"),
    ];

    let vehicle_types: &[(&str, &str)] = &[
        ("", "Any"),
        ("aircraft", "Aircraft"),
        ("helicopter", "Helicopter"),
        ("ship", "Ships"),
        ("tank", "Tanks"),
    ];

    rsx! {
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
            for (i , (value , label)) in countries.iter().enumerate() {
                ComboboxOption::<String> {
                    index: i,
                    value: value.to_string(),
                    text_value: label.to_string(),
                    {*label}
                }
            }
        }

        Combobox::<String> {
            style: "margin-left: 1vw;",
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
            for (i , (value , label)) in vehicle_types.iter().enumerate() {
                ComboboxOption::<String> {
                    index: i,
                    value: value.to_string(),
                    text_value: label.to_string(),
                    {*label}
                }
            }
        }

        Button {
            style: "margin-left: 1vw;",
            variant: ButtonVariant::Secondary,
            onclick: move |_| {
                let country = country_value.read().clone().unwrap_or_default();
                let vehicle = vehicle_type_value.read().clone().unwrap_or_default();

                spawn(async move {
                    match fetch_page(&country, &vehicle).await {
                        Ok(fetched_page) => {
                            page.set(fetched_page);
                            error_message.set(String::new());
                        }
                        Err(err) => {
                            error_message.set(format!("Erreur : {err}"));
                        }
                    }
                });
            },
            "Search",
        }

        if !error_message.read().is_empty() {
            p { style: "color: red;", "{error_message}" }
        }

        if *page.read() != Page::default() {
            ShowPage { page: page}
        }
    }
}

#[component]
pub fn ShowPage(page: Signal<Page>) -> Element {
    rsx! {
        // Un conteneur global pour espacer tes cartes si la page contient plusieurs skins
        div { style: "display: flex; flex-direction: column; gap: 2rem; padding: 20px 0;",

            // On boucle sur les données via un itérateur magique Dioxus
            {page.read().data.list.iter().map(|skin| rsx! {
                Card {
                    style: "margin: 0 auto; width: 100%; max-width: 80vw;",
                    key: "{skin.file.name}", // Toujours ajouter une clé unique pour optimiser le rendu des listes

                    // CardHeader {
                    //     CardTitle { "{skin.title}" }
                    // }

                    CardContent {
                        // Conteneur en ligne (Flex Row)
                        div { style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; gap: 1.5rem; width: 100%;",

                            // Bloc de gauche : Les statistiques et détails
                            div { style: "display: grid; gap: 0.5rem;",
                                ul {
                                    li { "Author : {skin.author.nickname}" }
                                    li { "Likes : {skin.likes}" }
                                    li { "Views: {skin.views}" }
                                    li { "Downloads : {skin.downloads}" }
                                    li { "Comments: {skin.comments}" }
                                    li {
                                        "Download:"
                                        ul {
                                            li { "Filename: {skin.file.name}" }
                                            li { "Size: {(skin.file.size as f32 / 1_000_000.0).round()} MB" }
                                        }
                                    }
                                }
                            }

                            // Bloc de droite : L'image (bien positionnée comme enfant direct du Flex Row)
                            img {
                                src: "{skin.get_thumbnail()}",
                                style: "max-width: 35%; height: auto; border-radius: 6px;"
                            }
                        }
                    }

                    CardFooter {
                        style: "flex-direction: column; gap: 0.5rem;",
                        Button {
                            variant: ButtonVariant::Secondary,
                            "Install Skin ({(skin.file.size as f32 / 1_000_000.0).round()} MB)"
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

#[component]
pub fn SkinCard(fetched_skin: Signal<Skin>) -> Element {
    rsx! {
        Separator {
            style: "margin: 25px auto; width: 50%;",
            horizontal: true,
            decorative: true,
        }

        Card {  style: "margin: 0 auto; width: 100%; max-width: 80vw;",
            // CardHeader {
            //     CardTitle { "{fetched_skin.read().title}"  }
            // }

            CardContent {
                div { style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; gap: 1.5rem; width: 100%;",

                    div { style: "display: grid; gap: 0.5rem;",
                        ul {
                            li { "Author : {fetched_skin.read().author.nickname}" }
                            li { "Likes : {fetched_skin.read().likes}" }
                            li { "Views: {fetched_skin.read().views}" }
                            li { "Downloads : {fetched_skin.read().downloads}" }
                            li { "Comments: {fetched_skin.read().comments}" }

                            li {
                                "Download:"
                                ul {
                                    li { "Filename: {fetched_skin.read().file.name}" }
                                        li {"Size: {(fetched_skin.read().file.size as f32 / 1e6 as f32).round()} MB" }

                                }
                            }
                        }
                    }
                    img {
                        src: "{fetched_skin.read().get_thumbnail()}",
                        style: "max-width: 35%; height: auto; border-radius: 6px;"
                    }
                }
            }

            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                Button { variant: ButtonVariant::Secondary, "Install Skin ({(fetched_skin.read().file.size as f32 / 1e6 as f32).round()} MB)" }
            }

        }
    }
}
