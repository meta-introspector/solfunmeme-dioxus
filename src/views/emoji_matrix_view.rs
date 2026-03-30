use dioxus::prelude::*;
use emoji_matrix_lib::{EmojiMatrix, EmojiMatrixEntry, parse_summary_total, parse_summary_root, rollup_emoji_matrix};
use log::info;
use dioxus_charts::BarChart;

#[component]
pub fn EmojiMatrixView() -> Element {
    let mut emoji_matrix = use_signal::<Option<EmojiMatrix>>(|| None);
    let mut log_scale_enabled = use_signal(|| false);
    let mut selected_category = use_signal(|| "All".to_string());

    let mut chart_series: Signal<Vec<Vec<f32>>> = use_signal(|| Vec::new());
    let mut chart_labels: Signal<Vec<String>> = use_signal(|| Vec::new());
    let mut categories: Signal<Vec<String>> = use_signal(|| Vec::new());

    use_future(move || async move {
        info!("Starting EmojiMatrixView data fetch.");
        let mut combined_entries: Vec<EmojiMatrixEntry> = Vec::new();

        // Read summary_total.txt
        let total_matrix = parse_summary_total();
        combined_entries.extend(total_matrix.entries);

        // Read summary_root.txt
        let root_matrix = parse_summary_root();
        combined_entries.extend(root_matrix.entries);

        let mut final_matrix = EmojiMatrix { entries: combined_entries };
        info!("Before rollup: {} entries", final_matrix.entries.len());
        final_matrix = rollup_emoji_matrix(final_matrix);
        info!("After rollup: {} entries", final_matrix.entries.len());

        // Prepare categories for filtering
        let unique_categories: std::collections::HashSet<String> = final_matrix.entries.iter()
            .flat_map(|entry| entry.emoji_counts.iter().map(|ec| ec.category.clone()))
            .collect::<std::collections::HashSet<String>>();
        let mut unique_categories_vec: Vec<String> = unique_categories.into_iter().collect();
        unique_categories_vec.sort();
        let mut all_categories = vec!["All".to_string()];
        all_categories.extend(unique_categories_vec);
        categories.set(all_categories);

        emoji_matrix.set(Some(final_matrix));
        info!("Emoji matrix set.");
    });

    use_effect(move || {
        let matrix_read = emoji_matrix.read();
        if let Some(matrix) = matrix_read.as_ref() {
            log::info!("Emoji matrix available in use_effect.");
            if let Some(total_entry) = matrix.entries.iter().find(|e| e.path == "total") {
                log::info!("Total entry found. Emoji counts: {}", total_entry.emoji_counts.len());
                if total_entry.emoji_counts.is_empty() {
                    log::error!("Total entry emoji_counts is empty.");
                }
                let current_series: Vec<f32> = total_entry.emoji_counts.iter()
                    .filter(|ec| {
                        let selected_cat = selected_category.read();
                        *selected_cat == "All" || ec.category == *selected_cat
                    })
                    .map(|ec| {
                        if *log_scale_enabled.read() {
                            (ec.count as f64).log10() as f32
                        } else {
                            ec.count as f32
                        }
                    })
                    .collect();
                chart_series.set(vec![current_series]);

                let current_labels: Vec<String> = total_entry.emoji_counts.iter()
                    .filter(|ec| {
                        let selected_cat = selected_category.read();
                        *selected_cat == "All" || ec.category == *selected_cat
                    })
                    .map(|ec| format!("{} {}", ec.emoji, ec.name))
                    .collect();
                chart_labels.set(current_labels);
                log::info!("Chart data updated.");
            } else {
                log::error!("Total entry not found in emoji matrix.");
            }
        } else {
            log::info!("Emoji matrix not yet available in use_effect.");
        }
    });

    rsx! {
        div {
            h1 { "Emoji Matrix" }
            h2 { class: "text-xl mb-4", "Frequency of Emoji Categories" }

            // Log Scale Toggle
            div {
                label {
                    input {
                        r#type: "checkbox",
                        checked: *log_scale_enabled.read(),
                        oninput: move |evt| log_scale_enabled.set(evt.checked()),
                    }
                    "Log Scale (base 10)"
                }
            }

            // Category Filter
            div {
                label { "Filter by Emoji Category:" }
                select {
                    onchange: move |evt| selected_category.set(evt.value()),
                    for cat in categories.read().iter() {
                        option {
                            value: "{cat}",
                            selected: *selected_category.read() == *cat,
                            "{cat}"
                        }
                    }
                }
            }

            if let Some(matrix) = emoji_matrix() {
                table {
                    class: "min-w-full divide-y divide-gray-200",
                    thead {
                        tr {
                            class: "bg-gray-50",
                            th {
                                class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Path (File System)"
                            }
                            th {
                                class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Emojis"
                            }
                        }
                    }
                    tbody {
                        class: "bg-white divide-y divide-gray-200",
                        for entry in &matrix.entries {
                            if entry.path == "total" {
                                tr {
                                    td { colspan: 2,
                                        if !chart_series.read().is_empty() && !chart_labels.read().is_empty() {
                                            div {
						style: "position: relative; display: flex; align-items: center;",
						div {
                                                    style: "transform: rotate(-90deg); transform-origin: center; position: absolute; left: -40px; font-size: 14px; color: #666;",
                                                    "Frequency"
                                            }
                                            if !chart_series.read().is_empty() && !chart_labels.read().is_empty() {
                                                BarChart {
                                                    padding_top: 30,
                                                    padding_left: 70,
                                                    padding_right: 50,
                                                    padding_bottom: 30,
                                                    bar_width: "8%",
                                                    series: chart_series.read().clone(),
                                                    labels: chart_labels.read().clone(),
                                                    viewbox_width: 800,
                                                    viewbox_height: 400,
                                                }
                                            } else {
                                                p { "Loading chart data..." }
                                            }
                                            }
                                        } else {
                                            p { "Loading chart data..." }
                                        }
                                    }
                                }
                            }
                            // Display table rows for other entries (not "total")
                            if entry.path != "total" {
                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900",
                                        "{entry.path.clone()}"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        for emoji_count in &entry.emoji_counts {
                                            if *selected_category.read() == "All" || emoji_count.category == *selected_category.read() {
                                                span {
                                                    class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 mr-2 mb-2",
                                                    "{emoji_count.emoji} {emoji_count.name} ({emoji_count.count})"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                p { "Loading..." }
            }
        }
    }
}
