use crate::api::networking::fetch_skin;
use crate::backend::config::Config;
use crate::components::button::*;
use crate::ui::camo_card_uninstall::CamoCardUninstall;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn UninstallPage() -> Element {
    let user_config = use_context::<Signal<Config>>();
    let client = use_context::<Signal<Client>>();

    let installed_skins_resource = use_resource(move || {
        let current_skins = user_config.read().installed_skins.clone(); // Will trigger a refresh if it changes
        let client_raw = client.read().clone();

        async move {
            let mut fetched_skins = Vec::new();
            for skin in current_skins {
                match fetch_skin(client_raw.clone(), skin.lang_group).await {
                    Ok(skin_data) => {
                        fetched_skins.push(skin_data);
                    }
                    Err(e) => {
                        dbg!("Error fetching installed skin: {}", e);
                    }
                }
            }
            fetched_skins
        }
    });

    let skins_guard = installed_skins_resource.value();
    let skins = skins_guard.as_ref();

    rsx! {
        div {
            class: "columns-1 sm:columns-2 lg:columns-3 xl:columns-4 gap-6 p-6 w-full max-w-[98vw] my-12 -mx-6",

            if let Some(installed_skins) = skins {
                for index in 0..installed_skins.len() {
                    CamoCardUninstall {
                        skin_signal: installed_skins_resource.map(move |v| {
                            v.as_ref()
                             .and_then(|list| list.get(index))
                             .expect("Skin index matching failed")
                        })
                    }
                }
            }

            Button {
                class: "z-10 fixed bottom-5 right-5",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    document::eval(
                        r#"
                        document.getElementById("inset").scrollTo({ top: 0, behavior: 'smooth' });
                        "#,
                    );
                },
                " Scroll to top",
            }
        }
    }
}
