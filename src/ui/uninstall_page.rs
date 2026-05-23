use crate::api::networking::fetch_skin;
use crate::api::structures::Skin;
use crate::backend::config::Config;
use crate::components::button::*;
use crate::ui::camo_card::CamoCard;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn UninstallPage() -> Element {
    let mut user_config = use_context::<Signal<Config>>();
    let client = use_context::<Signal<Client>>();
    let mut installed_skins: Signal<Vec<Skin>> = use_signal(|| Vec::new());

    use_hook(|| {
        // let mut installed_skins = installed_skins.read().clone();
        spawn(async move {
            for skin in &*user_config.read().installed_skins {
                match fetch_skin(client.read().clone(), skin.lang_group).await {
                    Ok(skin) => {
                        installed_skins.push(skin);
                    }
                    Err(e) => {
                        dbg!("Error fetching installed skin: {}", e);
                    }
                }
            }
        });
    });

    rsx! {
        div {
            style: "columns: 3 280px; gap: 1.5rem; padding: 25px; width: 100%; max-width: 98vw; margin: 3rem -1.5rem;",
            id: "camo-page",

            for index in 0..installed_skins.read().len() {
                CamoCard {
                    skin_signal: installed_skins.map(move |p| &p[index])
                }
            }

            Button {
                style: "z-index: 1; position: fixed; bottom: 1.25rem; right: 1.25rem;",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    document::eval(
                        r#"
                        window.scrollTo({ top: 0, behavior: 'smooth' });
                        "#,
                    );
                },
                " Scroll to top",
            }
        }
    }
}
