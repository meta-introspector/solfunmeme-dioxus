use dioxus::prelude::*;
use emoji_matrix_lib::{EmojiMatrix, EmojiMatrixEntry, EmojiCount, parse_summary_total, parse_summary_root, rollup_emoji_matrix};
use log::info;
use dioxus_charts::BarChart;

#[component]
pub fn EmojiMatrixView() -> Element {
    let mut emoji_matrix = use_signal::<Option<EmojiMatrix>>(|| None);

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

        emoji_matrix.set(Some(final_matrix));
        info!("Emoji matrix set.");
    });

    rsx! {
        div {
            h1 { "Emoji Matrix" }
            if let Some(matrix) = emoji_matrix() {
                table {
                    class: "min-w-full divide-y divide-gray-200",
                    thead {
                        tr {
                            class: "bg-gray-50",
                            th {
                                class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Path"
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
                            tr {
                                td {
                                    class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900",
                                    "{entry.path.clone()}"
                                }
                                td {
                                    class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    for emoji_count in &entry.emoji_counts {
                                        span {
                                            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 mr-2 mb-2",
                                            "{emoji_count.emoji} {emoji_count.name} ({emoji_count.count})"
                                        }
                                    }
                                }
                            }
                            if entry.path == "total" {
                                tr {
                                    td { colspan: 2,
                                        BarChart {
                                            padding_top: 30,
                                            padding_left: 70,
                                            padding_right: 50,
                                            padding_bottom: 30,
                                            bar_width: "8%",
                                            series: vec![
                                                entry.emoji_counts.iter().map(|ec| ec.count as f32).collect(),
                                            ],
                                            labels: entry.emoji_counts.iter().map(|ec| format!("{}: {}", ec.emoji, ec.name)).collect::<Vec<String>>(),
                                            viewbox_width: 800,
                                            viewbox_height: 400,
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
