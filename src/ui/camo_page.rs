use crate::api::networking::fetch_page;
use crate::api::structures::*;
use crate::backend::structures::SearchParams;
use crate::components::button::*;
use crate::components::pagination::*;
use crate::ui::filters::FiltersModal;
use crate::ui::install_card::CamoCard;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn CamoPage() -> Element {
    let client = use_context::<Signal<Client>>();

    let mut page_data = use_signal(|| Page::default());
    let mut active_page = use_signal(|| 0);

    let mut active_params = use_signal(|| SearchParams::default());

    use_resource(move || {
        let params = active_params.read().clone();
        tracing::debug!("Fetch page params: {:?}", params);
        let page_num = active_page.read().clone();
        let client_clone = client.read().clone();

        let country = params.country.unwrap_or("".to_string());
        let class = params.class.unwrap_or("".to_string());
        let v_type = params.v_type.unwrap_or("".to_string());
        let vehicle = params.vehicle.unwrap_or("".to_string());

        async move {
            match fetch_page(client_clone, &country, &v_type, &class, &vehicle, page_num).await {
                Ok(fetched_page) => {
                    page_data.set(fetched_page);
                }
                Err(err) => {
                    tracing::debug!("Error fetching page: {}", err);
                }
            }
        }
    });

    rsx! {

        FiltersModal {
            on_search: move |new_params: SearchParams| {
                active_params.set(new_params);
                active_page.set(0);
            }
        }

        // Fetched feed page
        div {
            class: "columns-1 sm:columns-2 lg:columns-3 xl:columns-4 gap-6 p-6 w-full max-w-[98vw] my-12 -mx-6",
            id: "camo-page",

            for index in 0..page_data.read().data.list.len() {
                CamoCard {
                    skin_signal: page_data.map(move |p| &p.data.list[index])
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

        Pagination {
            style: "position: fixed; bottom: 1.25rem; margin: 0 auto;",
            PaginationContent {
                style: "background-color: #0a0a0a; border-radius: 10px; padding: 0.3rem;",
                PaginationItem {
                    PaginationPrevious {
                        onclick: move |_| {
                            let current = *active_page.read();
                            if current > 0 {
                                active_page.set(current - 1);
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
                            let current = *active_page.read();
                            active_page.set(current + 1);
                        }
                    }
                }
            }
        }
    }
}
