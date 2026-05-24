use crate::api::structures::*;
use crate::components::button::*;
use crate::ui::camo_card::CamoCard;
use dioxus::prelude::*;

#[component]
pub fn CamoPage(page: Signal<Page>) -> Element {
    rsx! {
        div {
            style: "columns: 3 280px; gap: 1.5rem; padding: 25px; width: 100%; max-width: 98vw; margin: 3rem -1.5rem;",
            id: "camo-page",

            for index in 0..page.read().data.list.len() {
                CamoCard {
                    skin_signal: page.map(move |p| &p.data.list[index])
                }
            }

            Button {
                style: "z-index: 1; position: fixed; bottom: 1.25rem; right: 1.25rem;",
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
