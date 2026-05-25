use crate::api::structures::*;
use crate::components::button::*;
use crate::ui::camo_card::CamoCard;
use dioxus::prelude::*;

#[component]
pub fn CamoPage(page: Signal<Page>) -> Element {
    rsx! {
        div {
            class: "columns-1 sm:columns-2 lg:columns-3 xl:columns-4 gap-6 p-6 w-full max-w-[98vw] my-12 -mx-6",
            id: "camo-page",

            for index in 0..page.read().data.list.len() {
                CamoCard {
                    skin_signal: page.map(move |p| &p.data.list[index])
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
