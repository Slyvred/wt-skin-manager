use crate::api::networking::*;
use crate::api::structures::*;
use crate::components::button::*;
use crate::components::combobox::*;
use crate::components::pagination::*;
use crate::ui::camo_page::CamoPage;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn CamoFeed() -> Element {
    let client = use_signal(|| Client::new());

    let mut vehicle_country_query = use_signal(String::new);
    let mut vehicle_country_value = use_signal(|| None::<String>);

    let mut vehicle_type_query = use_signal(String::new);
    let mut vehicle_type_value = use_signal(|| None::<String>);

    let mut vehicle_class_query = use_signal(String::new);
    let mut vehicle_class_value = use_signal(|| None::<String>);

    let mut vehicle_query = use_signal(String::new);
    let mut vehicle_value = use_signal(|| None::<String>);

    let mut page = use_signal(|| Page::default());
    let mut error_message = use_signal(|| String::new());

    let mut filters = use_signal(|| Filters::default());
    let mut active_page = use_signal(|| 0);

    let search_action = move |target_page: i32| {
        spawn(async move {
            match fetch_page(
                client.read().clone(),
                vehicle_country_value.read().as_deref().unwrap_or_default(),
                vehicle_type_value.read().as_deref().unwrap_or_default(),
                vehicle_class_value.read().as_deref().unwrap_or_default(),
                vehicle_value.read().as_deref().unwrap_or_default(),
                target_page,
            )
            .await
            {
                Ok(fetched_page) => {
                    page.set(fetched_page);
                }
                Err(err) => {
                    error_message.set(format!("Error : {err}"));
                }
            }
        });
    };

    use_hook(move || {
        spawn(async move {
            match fetch_filters(client.read().clone()).await {
                Ok(fetched_filters) => {
                    // let _ = dbg!("{:?}", &fetched_filters);
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
            style: "position: fixed; top: 1.25rem;",
            Combobox::<String> {
                value: Some(vehicle_country_value.into()),
                on_value_change: move |next: Option<String>| {
                    let update = {
                        let current = vehicle_country_value.read();
                        *current != next
                    };
                    if update {
                        vehicle_country_value.set(next);
                    }
                },
                query: Some(vehicle_country_query()),
                on_query_change: move |next| vehicle_country_query.set(next),
                placeholder: "Country",
                aria_label: "Country",
                list_aria_label: "Countries",
                ComboboxEmpty { "No country found." }

                {
                    filters.read().vehicle_country.variants.iter().enumerate().map(|(i, variant)| {
                        let raw_value = variant.value.as_deref().unwrap_or("");
                        let name = &variant.name;
                        let count = variant.count.unwrap_or_default();

                        let final_value = if raw_value == "any" { "" } else { raw_value };
                        let label = if count == 0 { format!("{}", name) } else { format!("{} ({})", name, count) };

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{raw_value}_{i}",
                                index: i,
                                value: final_value.to_string(),
                                text_value: name.to_string(),
                                "{label}"
                            }
                        }
                    })
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(vehicle_type_value.into()),
                on_value_change: move |next: Option<String>| {
                    let update = {
                        let current = vehicle_type_value.read();
                        *current != next
                    };
                    if update {
                        vehicle_type_value.set(next);
                    }
                },
                query: Some(vehicle_type_query()),
                on_query_change: move |next| vehicle_type_query.set(next),
                placeholder: "Type",
                aria_label: "Type",
                list_aria_label: "Types",
                ComboboxEmpty { "No type found." }

                {
                    filters.read().vehicle_type.variants.iter().enumerate().map(|(i, variant)| {
                        let raw_value = variant.value.as_deref().unwrap_or("");
                        let name = &variant.name;
                        let count = variant.count.unwrap_or_default();

                        let final_value = if raw_value == "any" { "" } else { raw_value };
                        let label = if count == 0 { format!("{}", name) } else { format!("{} ({})", name, count) };

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{raw_value}_{i}",
                                index: i,
                                value: final_value.to_string(),
                                text_value: name.to_string(),
                                "{label}"
                            }
                        }
                    })
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(vehicle_class_value.into()),
                on_value_change: move |next: Option<String>| {
                    let update = {
                        let current = vehicle_class_value.read();
                        *current != next
                    };
                    if update {
                        vehicle_class_value.set(next);
                    }
                },
                query: Some(vehicle_class_query()),
                on_query_change: move |next| vehicle_class_query.set(next),
                placeholder: "Class",
                aria_label: "Class",
                list_aria_label: "Classes",
                ComboboxEmpty { "No classes found." }

                {
                    filters
                        .read()
                        .vehicle_class
                        .variants
                        .iter()
                        .filter(|class| {

                            if class.value.as_deref() == Some("any") {
                                return true;
                            }

                            let selected_type = vehicle_type_value.read();
                            let current_type_str = selected_type.as_deref().unwrap_or("");

                            if current_type_str.is_empty() {
                                return true;
                            }

                            if let Some(dep_type) = &class.dep {
                                if let Some(allowed_types) = &dep_type.vehicle_type {
                                    return allowed_types.iter().any(|t| t == current_type_str);
                                }
                            }
                            false
                        })
                        .enumerate()
                        .map(|(i, variant)| {
                            let raw_value = variant.value.as_deref().unwrap_or("");
                            let name = &variant.name;
                            let count = variant.count.unwrap_or_default();

                            let final_value = if raw_value == "any" { "" } else { raw_value };
                            let label = if count == 0 { format!("{}", name) } else { format!("{} ({})", name, count) };

                            rsx! {
                                ComboboxOption::<String> {
                                    key: "{raw_value}_{i}",
                                    index: i,
                                    value: final_value.to_string(),
                                    text_value: name.to_string(),
                                    "{label}"
                                }
                            }
                        })
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(vehicle_value.into()),
                on_value_change: move |next: Option<String>| {
                    let update = {
                        let current = vehicle_value.read();
                        *current != next
                    };
                    if update {
                        vehicle_value.set(next);
                    }
                },
                query: Some(vehicle_query()),
                on_query_change: move |next| vehicle_query.set(next),
                placeholder: "Vehicle",
                aria_label: "Vehicle",
                list_aria_label: "Vehicles",
                ComboboxEmpty { "No vehicles found." }

                {
                    filters
                        .read()
                        .vehicle
                        .variants
                        .iter()
                        .filter(|class| {
                            if class.value.as_deref() == Some("any") {
                                return true;
                            }

                            let type_str = vehicle_type_value.read().as_deref().unwrap_or("").to_string();
                            let country_str = vehicle_country_value.read().as_deref().unwrap_or("").to_string();
                            let class_str = vehicle_class_value.read().as_deref().unwrap_or("").to_string();

                            if type_str.is_empty() && country_str.is_empty() && class_str.is_empty() {
                                return false;
                            }

                            if let Some(dep_type) = &class.dep {

                                let type_match = type_str.is_empty() || dep_type.vehicle_type.as_ref()
                                    .map(|types| types.iter().any(|t| t == &type_str))
                                    .unwrap_or(false);

                                let country_match = country_str.is_empty() || dep_type.vehicle_country.as_ref()
                                    .map(|countries| countries.iter().any(|c| c == &country_str))
                                    .unwrap_or(false);

                                let class_match = class_str.is_empty() || dep_type.vehicle_class.as_ref()
                                    .map(|classes| classes.iter().any(|cl| cl == &class_str))
                                    .unwrap_or(false);

                                return type_match && country_match && class_match;
                            }

                            false
                        })
                        .enumerate()
                        .map(|(i, variant)| {
                            let raw_value = variant.value.as_deref().unwrap_or("");
                            let name = &variant.name;
                            let count = variant.count.unwrap_or_default();

                            let final_value = if raw_value == "any" { "" } else { raw_value };

                            let label = if count == 0 {
                                format!("{}", name)
                            } else {
                                format!("{} ({})", name, count)
                            };

                            rsx! {
                                ComboboxOption::<String> {
                                    key: "{raw_value}_{i}",
                                    index: i,
                                    value: final_value.to_string(),
                                    text_value: name.to_string(),
                                    "{label}"
                                }
                            }
                        })
                }
            }

            Button {
                style: "margin-left: 0.5rem;",
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
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
            CamoPage { page: page }
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
