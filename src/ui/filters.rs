use crate::api::networking::fetch_filters;
use crate::api::structures::Filters;
use crate::backend::structures::SearchParams;
use crate::components::button::*;
use crate::components::combobox::*;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn FiltersModal(on_search: EventHandler<SearchParams>) -> Element {
    let client = use_context::<Signal<Client>>();
    let mut filters = use_signal(|| Filters::default());
    let mut error_message = use_signal(|| String::new());

    let mut country_query = use_signal(String::new);
    let mut country_value = use_signal(|| None::<String>);

    let mut type_query = use_signal(String::new);
    let mut type_value = use_signal(|| None::<String>);

    let mut class_query = use_signal(String::new);
    let mut class_value = use_signal(|| None::<String>);

    let mut vehicle_query = use_signal(String::new);
    let mut vehicle_value = use_signal(|| None::<String>);

    use_hook(move || {
        spawn(async move {
            match fetch_filters(client.read().clone()).await {
                Ok(fetched_filters) => {
                    // tracing::debug!("{:?}", &fetched_filters);
                    filters.set(fetched_filters);
                    error_message.set(String::new());
                }
                Err(err) => {
                    error_message.set(format!("Error fetching filters: {err}"));
                }
            }
        });
    });

    rsx! {
        div {
            style: "position: fixed; top: 1.25rem;",
            Combobox::<String> {
                value: Some(country_value.into()),
                on_value_change: move |next: Option<String>| country_value.set(next),
                query: country_query(),
                on_query_change: move |next| country_query.set(next),
                placeholder: "Country",
                aria_label: "Country",
                list_aria_label: "Countries",
                ComboboxEmpty { "No country found." }

                {
                    filters.read().vehicle_country.variants.iter().enumerate().map(|(i, variant)| {
                        let raw_value = variant.value.as_deref().unwrap_or("");
                        let final_value = if raw_value == "any" { "" } else { raw_value };

                        let count = variant.count.unwrap_or_default();

                        let label: String;

                        if count == 0 {
                           label = variant.name.clone()
                        } else {
                            label = format!("{} ({})", variant.name, count)
                        }

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{raw_value}_{i}",
                                index: i,
                                value: final_value.to_string(),
                                text_value: variant.name.clone(),
                                "{label}"
                            }
                        }
                    })
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(type_value.into()),
                on_value_change: move |next: Option<String>| type_value.set(next),
                query: type_query(),
                on_query_change: move |next| type_query.set(next),
                placeholder: "Type",
                aria_label: "Type",
                list_aria_label: "Types",
                ComboboxEmpty { "No type found." }

                {
                    filters.read().vehicle_type.variants.iter().enumerate().map(|(i, variant)| {
                        let raw_value = variant.value.as_deref().unwrap_or("");
                        let final_value = if raw_value == "any" { "" } else { raw_value };

                        let count = variant.count.unwrap_or_default();

                        let label: String;

                        if count == 0 {
                           label = variant.name.clone()
                        } else {
                            label = format!("{} ({})", variant.name, count)
                        }

                        rsx! {
                            ComboboxOption::<String> {
                                key: "{raw_value}_{i}",
                                index: i,
                                value: final_value.to_string(),
                                text_value: variant.name.clone(),
                                "{label}"
                            }
                        }
                    })
                }
            }

            Combobox::<String> {
                style: "margin-left: 0.5rem;",
                value: Some(class_value.into()),
                on_value_change: move |next: Option<String>| class_value.set(next),
                query: class_query(),
                on_query_change: move |next| class_query.set(next),
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
                            // If there's no class selected we return all of them
                            if class.value.as_deref() == Some("any") {
                                return true;
                            }

                            let current_type: String = type_value.read().clone().unwrap_or_default();

                            // Same if no vehicule type is selected
                            if current_type.is_empty() {
                                return true;
                            }

                            // If there's some dependencies
                            if let Some(dep_type) = &class.dep {
                                // We only return the classes allowed associated withe the vehicule type
                                if let Some(allowed_types) = &dep_type.vehicle_type {
                                    return allowed_types.iter().any(|t| t == &current_type);
                                }
                            }
                            false
                        })
                        .enumerate()
                        .map(|(i, variant)| {
                            let raw_value = variant.value.as_deref().unwrap_or_default();
                            let name = &variant.name;
                            let count = variant.count.unwrap_or_default();
                            let final_value = if raw_value == "any" { "" } else { raw_value };

                            let label: String;

                            if count == 0 {
                               label = variant.name.clone()
                            } else {
                                label = format!("{} ({})", variant.name, count)
                            }


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
                on_value_change: move |next: Option<String>| vehicle_value.set(next),
                query: vehicle_query(),
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
                        .filter(|variant| {

                            let current_type = type_value.read().clone().unwrap_or_default();
                            let current_country = country_value.read().clone().unwrap_or_default();
                            let current_class = class_value.read().clone().unwrap_or_default();

                            if variant.value.as_deref() == Some("any") {
                                return true;
                            }

                            // If no filter is selected we hide all the vehicle (to avoid lag)
                            if current_type.is_empty() && current_country.is_empty() && current_class.is_empty() {
                                return false;
                            }

                            // If our vehicle has no dependency hide vehicles
                            let Some(dep) = &variant.dep else { return false };

                            // Helper closure to check if a specific filter constraint is satisfied
                            // Returns true if the filter is empty OR if the current selection is in the allowed list
                            let is_match = |current_val: &str, allowed_list: &Option<Vec<String>>| {
                                current_val.is_empty() || allowed_list.as_ref().is_some_and(|list| list.iter().any(|v| v == current_val))
                            };

                            return is_match(&current_type, &dep.vehicle_type) && is_match(&current_country, &dep.vehicle_country) && is_match(&current_class, &dep.vehicle_class);
                        })
                        .enumerate()
                        .map(|(i, variant)| {
                            let raw_value = variant.value.as_deref().unwrap_or("");
                            let name = &variant.name;
                            let count = variant.count.unwrap_or_default();

                            let final_value = if raw_value == "any" { "" } else { raw_value };

                            let label: String;
                            if count == 0 {
                                label = format!("{}", name)
                            } else {
                                label = format!("{} ({})", name, count)
                            }

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
                    on_search.call(SearchParams {
                        country: country_value.read().clone(),
                        v_type: type_value.read().clone(),
                        class: class_value.read().clone(),
                        vehicle: vehicle_value.read().clone(),
                    });
                },
                "Search",
            }

            if !error_message.read().is_empty() {
                p { style: "color: red;", "{error_message}" }
            }
        }
    }
}
