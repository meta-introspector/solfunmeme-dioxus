use dioxus::prelude::*;
use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
struct CoverageEntry {
    filename: String,
    regions: (usize, usize, f64), // (total, missed, cover %)
    functions: (usize, usize, f64),
    lines: (usize, usize, f64),
    branches: (usize, usize, f64),
}

#[derive(Clone, PartialEq, Copy)]
enum SortColumn {
    Filename,
    Regions,
    Functions,
    Lines,
    Branches,
}

#[derive(Clone, PartialEq, Copy)]
enum SortDirection {
    Ascending,
    Descending,
}

#[component]
pub fn CoverageApp() -> Element {
    let mut filter_text = use_signal(|| String::new());
    let mut show_low_coverage = use_signal(|| false);
    let sort_column = use_signal(|| SortColumn::Filename);
    let sort_direction = use_signal(|| SortDirection::Ascending);

    let coverage_data = use_memo(move || {
        let mut entries = get_coverage_data();
        // Apply filter
        entries.retain(|entry| {
            entry
                .filename
                .to_lowercase()
                .contains(&filter_text().to_lowercase())
                && (!show_low_coverage() || entry.regions.2 < 50.0)
        });
        // Apply sort
        entries.sort_by(|a, b| {
            let order = match sort_column() {
                SortColumn::Filename => a.filename.cmp(&b.filename),
                SortColumn::Regions => a
                    .regions
                    .2
                    .partial_cmp(&b.regions.2)
                    .unwrap_or(Ordering::Equal),
                SortColumn::Functions => a
                    .functions
                    .2
                    .partial_cmp(&b.functions.2)
                    .unwrap_or(Ordering::Equal),
                SortColumn::Lines => a.lines.2.partial_cmp(&b.lines.2).unwrap_or(Ordering::Equal),
                SortColumn::Branches => a
                    .branches
                    .2
                    .partial_cmp(&b.branches.2)
                    .unwrap_or(Ordering::Equal),
            };
            if sort_direction() == SortDirection::Descending {
                order.reverse()
            } else {
                order
            }
        });
        entries
    });

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6 text-center",
                "Code Coverage Report"
            }

            // Filter and Toggle Controls
            div { class: "mb-6 flex flex-col sm:flex-row gap-4",
                input {
                    class: "w-full sm:w-64 p-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                    placeholder: "Filter by filename...",
                    value: "{filter_text}",
                    oninput: move |evt| filter_text.set(evt.value().clone())
                }
                label { class: "flex items-center gap-2",
                    input {
                        r#type: "checkbox",
                        checked: "{show_low_coverage}",
                        onchange: move |evt| show_low_coverage.set(evt.checked())
                    }
                    span { class: "text-sm text-gray-700", "Show only low coverage (<50%)" }
                }
            }

            // Coverage Table
            div { class: "bg-white shadow-lg rounded-lg overflow-x-auto",
                table { class: "w-full table-auto",
                    thead { class: "bg-gray-100",
                        tr {
                            th {
                                class: "px-4 py-2 text-left text-sm font-semibold text-gray-700 cursor-pointer",
                                onclick: move |_| toggle_sort(SortColumn::Filename, sort_column, sort_direction),
                                "Filename"
                                {sort_indicator(SortColumn::Filename, sort_column(), sort_direction())}
                            }
                            th {
                                class: "px-4 py-2 text-left text-sm font-semibold text-gray-700 cursor-pointer",
                                onclick: move |_| toggle_sort(SortColumn::Regions, sort_column, sort_direction),
                                "Regions"
                                {sort_indicator(SortColumn::Regions, sort_column(), sort_direction())}
                            }
                            th {
                                class: "px-4 py-2 text-left text-sm font-semibold text-gray-700 cursor-pointer",
                                onclick: move |_| toggle_sort(SortColumn::Functions, sort_column, sort_direction),
                                "Functions"
                                {sort_indicator(SortColumn::Functions, sort_column(), sort_direction())}
                            }
                            th {
                                class: "px-4 py-2 text-left text-sm font-semibold text-gray-700 cursor-pointer",
                                onclick: move |_| toggle_sort(SortColumn::Lines, sort_column, sort_direction),
                                "Lines"
                                {sort_indicator(SortColumn::Lines, sort_column(), sort_direction())}
                            }
                            th {
                                class: "px-4 py-2 text-left text-sm font-semibold text-gray-700 cursor-pointer",
                                onclick: move |_| toggle_sort(SortColumn::Branches, sort_column, sort_direction),
                                "Branches"
                                {sort_indicator(SortColumn::Branches, sort_column(), sort_direction())}
                            }
                        }
                    }
                    tbody {
                        for entry in coverage_data() {
                            tr { class: "border-t",
                                td { class: "px-4 py-2 text-sm text-gray-700", "{entry.filename}" }
                                td { class: "px-4 py-2 text-sm",
                                    span { class: coverage_class(entry.regions.2),
                                        "{entry.regions.0} / {entry.regions.1} ({entry.regions.2:.2}%)"
                                    }
                                }
                                td { class: "px-4 py-2 text-sm",
                                    span { class: coverage_class(entry.functions.2),
                                        "{entry.functions.0} / {entry.functions.1} ({entry.functions.2:.2}%)"
                                    }
                                }
                                td { class: "px-4 py-2 text-sm",
                                    span { class: coverage_class(entry.lines.2),
                                        "{entry.lines.0} / {entry.lines.1} ({entry.lines.2:.2}%)"
                                    }
                                }
                                td { class: "px-4 py-2 text-sm",
                                    span { class: coverage_class(entry.branches.2),
                                        "{entry.branches.0} / {entry.branches.1} ({entry.branches.2:.2}%)"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Summary
            div { class: "mt-6 bg-white shadow-lg rounded-lg p-6",
                h2 { class: "text-2xl font-semibold mb-4", "Summary" }
                p { class: "text-sm text-gray-700",
                    "Total Regions: {coverage_data().iter().map(|e| e.regions.0).sum::<usize>()} / {coverage_data().iter().map(|e| e.regions.1).sum::<usize>()} ({(coverage_data().iter().fold(0.0, |acc, e| acc + e.regions.2 * e.regions.0 as f64) / coverage_data().iter().map(|e| e.regions.0 as f64).sum::<f64>()):.2}%)"
                }
                p { class: "text-sm text-gray-700",
                    "Total Functions: {coverage_data().iter().map(|e| e.functions.0).sum::<usize>()} / {coverage_data().iter().map(|e| e.functions.1).sum::<usize>()} ({(coverage_data().iter().fold(0.0, |acc, e| acc + e.functions.2 * e.functions.0 as f64) / coverage_data().iter().map(|e| e.functions.0 as f64).sum::<f64>()):.2}%)"
                }
                p { class: "text-sm text-gray-700",
                    "Total Lines: {coverage_data().iter().map(|e| e.lines.0).sum::<usize>()} / {coverage_data().iter().map(|e| e.lines.1).sum::<usize>()} ({(coverage_data().iter().fold(0.0, |acc, e| acc + e.lines.2 * e.lines.0 as f64) / coverage_data().iter().map(|e| e.lines.0 as f64).sum::<f64>()):.2}%)"
                }
            }
        }
    }
}

fn coverage_class(coverage: f64) -> &'static str {
    if coverage < 50.0 {
        "text-red-700 font-semibold"
    } else if coverage < 80.0 {
        "text-yellow-700 font-semibold"
    } else {
        "text-green-700 font-semibold"
    }
}

fn sort_indicator(
    column: SortColumn,
    current_column: SortColumn,
    direction: SortDirection,
) -> Element {
    if column == current_column {
        rsx! {
            span { class: "ml-1",
                match direction {
                    SortDirection::Ascending => "↑",
                    SortDirection::Descending => "↓",
                }
            }
        }
    } else {
        rsx! { span {} }
    }
}

fn toggle_sort(
    column: SortColumn,
    mut sort_column: Signal<SortColumn>,
    mut sort_direction: Signal<SortDirection>,
) {
    if *sort_column.read() == column {
        let new_direction = match *sort_direction.read() {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        };
        sort_direction.set(new_direction);

    //sort_direction.set(match *sort_direction.read() {
    //        sort_direction.write_with(|dir| match dir {
    //            SortDirection::Ascending => SortDirection::Descending,
    //            SortDirection::Descending => SortDirection::Ascending,
    //        });
    } else {
        sort_column.set(column);
        sort_direction.set(SortDirection::Ascending);
    }
}

fn get_coverage_data() -> Vec<CoverageEntry> {
    vec![
        CoverageEntry {
            filename: "app.rs".to_string(),
            regions: (19, 19, 0.00),
            functions: (3, 3, 0.00),
            lines: (11, 11, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "fetch_parser.rs".to_string(),
            regions: (226, 226, 0.00),
            functions: (17, 17, 0.00),
            lines: (134, 134, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "fetch_util.rs".to_string(),
            regions: (112, 112, 0.00),
            functions: (12, 12, 0.00),
            lines: (68, 68, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "header.rs".to_string(),
            regions: (162, 162, 0.00),
            functions: (33, 33, 0.00),
            lines: (115, 115, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "main.rs".to_string(),
            regions: (2, 2, 0.00),
            functions: (1, 1, 0.00),
            lines: (2, 2, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\accountstate.rs".to_string(),
            regions: (14, 14, 0.00),
            functions: (4, 4, 0.00),
            lines: (12, 12, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\adaptercluster.rs".to_string(),
            regions: (187, 172, 8.02),
            functions: (16, 12, 25.00),
            lines: (128, 112, 12.50),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\cluster_store.rs".to_string(),
            regions: (48, 0, 100.00),
            functions: (10, 0, 100.00),
            lines: (40, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\cluster_store_test.rs".to_string(),
            regions: (182, 0, 100.00),
            functions: (11, 0, 100.00),
            lines: (70, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\crypto.rs".to_string(),
            regions: (399, 74, 81.45),
            functions: (31, 14, 54.84),
            lines: (210, 29, 86.19),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\crypto_test.rs".to_string(),
            regions: (252, 0, 100.00),
            functions: (17, 0, 100.00),
            lines: (125, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\binder.rs".to_string(),
            regions: (6, 3, 50.00),
            functions: (2, 1, 50.00),
            lines: (12, 6, 50.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\binder_test.rs".to_string(),
            regions: (40, 0, 100.00),
            functions: (4, 0, 100.00),
            lines: (28, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\constants_test.rs".to_string(),
            regions: (75, 0, 100.00),
            functions: (4, 0, 100.00),
            lines: (30, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\emoji_tests.rs".to_string(),
            regions: (226, 0, 100.00),
            functions: (7, 0, 100.00),
            lines: (150, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\emojis.rs".to_string(),
            regions: (180, 32, 82.22),
            functions: (3, 1, 66.67),
            lines: (79, 12, 84.81),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\level.rs".to_string(),
            regions: (70, 29, 58.57),
            functions: (11, 1, 90.91),
            lines: (31, 8, 74.19),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\level_test.rs".to_string(),
            regions: (89, 0, 100.00),
            functions: (7, 0, 100.00),
            lines: (55, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\style.rs".to_string(),
            regions: (202, 202, 0.00),
            functions: (40, 40, 0.00),
            lines: (212, 212, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\lean\\types\\simple_expr_type.rs".to_string(),
            regions: (28, 28, 0.00),
            functions: (2, 2, 0.00),
            lines: (69, 69, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\memes.rs".to_string(),
            regions: (3, 3, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\metameme.rs".to_string(),
            regions: (22, 22, 0.00),
            functions: (7, 7, 0.00),
            lines: (22, 22, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\mycluster.rs".to_string(),
            regions: (61, 57, 6.56),
            functions: (5, 4, 20.00),
            lines: (44, 40, 9.09),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\notificationinfo.rs".to_string(),
            regions: (96, 0, 100.00),
            functions: (12, 0, 100.00),
            lines: (55, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\simple_expr.rs".to_string(),
            regions: (378, 265, 29.89),
            functions: (36, 28, 22.22),
            lines: (361, 221, 38.78),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\storage.rs".to_string(),
            regions: (4, 4, 0.00),
            functions: (2, 2, 0.00),
            lines: (2, 2, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\tokenaccountresponse.rs".to_string(),
            regions: (15, 15, 0.00),
            functions: (4, 4, 0.00),
            lines: (18, 18, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "model\\use_connections.rs".to_string(),
            regions: (187, 187, 0.00),
            functions: (22, 22, 0.00),
            lines: (117, 117, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "password_manager.rs".to_string(),
            regions: (370, 370, 0.00),
            functions: (50, 50, 0.00),
            lines: (249, 249, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "playground\\app.rs".to_string(),
            regions: (36, 36, 0.00),
            functions: (24, 24, 0.00),
            lines: (30, 30, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "playground\\test_app.rs".to_string(),
            regions: (497, 497, 0.00),
            functions: (67, 67, 0.00),
            lines: (502, 502, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "state\\use_memes.rs".to_string(),
            regions: (64, 64, 0.00),
            functions: (8, 8, 0.00),
            lines: (34, 34, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "svg_assets.rs".to_string(),
            regions: (120, 120, 0.00),
            functions: (60, 60, 0.00),
            lines: (120, 120, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "utils.rs".to_string(),
            regions: (102, 102, 0.00),
            functions: (13, 13, 0.00),
            lines: (65, 65, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\accounts.rs".to_string(),
            regions: (94, 94, 0.00),
            functions: (22, 22, 0.00),
            lines: (64, 64, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\airdrop.rs".to_string(),
            regions: (36, 36, 0.00),
            functions: (6, 6, 0.00),
            lines: (26, 26, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\clusters.rs".to_string(),
            regions: (133, 133, 0.00),
            functions: (19, 19, 0.00),
            lines: (80, 80, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\clusters_management.rs".to_string(),
            regions: (98, 98, 0.00),
            functions: (8, 8, 0.00),
            lines: (56, 56, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\coins.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\component_memes\\mod.rs".to_string(),
            regions: (72, 72, 0.00),
            functions: (5, 5, 0.00),
            lines: (72, 72, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connect_first.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connection_buttons.rs".to_string(),
            regions: (10, 10, 0.00),
            functions: (4, 4, 0.00),
            lines: (4, 4, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connection_filter.rs".to_string(),
            regions: (10, 10, 0.00),
            functions: (3, 3, 0.00),
            lines: (12, 12, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connection_list.rs".to_string(),
            regions: (21, 21, 0.00),
            functions: (3, 3, 0.00),
            lines: (15, 15, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connection_management.rs".to_string(),
            regions: (50, 50, 0.00),
            functions: (6, 6, 0.00),
            lines: (34, 34, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\connections.rs".to_string(),
            regions: (16, 16, 0.00),
            functions: (2, 2, 0.00),
            lines: (8, 8, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\core_buttons.rs".to_string(),
            regions: (16, 16, 0.00),
            functions: (6, 6, 0.00),
            lines: (6, 6, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_buttons.rs".to_string(),
            regions: (10, 10, 0.00),
            functions: (4, 4, 0.00),
            lines: (4, 4, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend\\app.rs".to_string(),
            regions: (2, 0, 100.00),
            functions: (2, 0, 100.00),
            lines: (2, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend\\components.rs".to_string(),
            regions: (17, 11, 35.29),
            functions: (9, 3, 66.67),
            lines: (9, 3, 66.67),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend\\forms.rs".to_string(),
            regions: (122, 120, 1.64),
            functions: (13, 11, 15.38),
            lines: (70, 68, 2.86),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend\\tests.rs".to_string(),
            regions: (88, 0, 100.00),
            functions: (7, 0, 100.00),
            lines: (51, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend\\validation.rs".to_string(),
            regions: (46, 14, 69.57),
            functions: (2, 0, 100.00),
            lines: (48, 14, 70.83),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\crypto_frontend_test.rs".to_string(),
            regions: (47, 0, 100.00),
            functions: (5, 0, 100.00),
            lines: (36, 0, 100.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\dashboard.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\encryption.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\expression_parsing.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\extras.rs".to_string(),
            regions: (2, 2, 0.00),
            functions: (2, 2, 0.00),
            lines: (2, 2, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\extras_views\\sign_message.rs".to_string(),
            regions: (17, 17, 0.00),
            functions: (3, 3, 0.00),
            lines: (17, 17, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\extras_views\\sign_tx.rs".to_string(),
            regions: (47, 47, 0.00),
            functions: (3, 3, 0.00),
            lines: (30, 30, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\extras_views\\siws.rs".to_string(),
            regions: (30, 30, 0.00),
            functions: (4, 4, 0.00),
            lines: (25, 25, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\footer.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\lean.rs".to_string(),
            regions: (5, 5, 0.00),
            functions: (2, 2, 0.00),
            lines: (5, 5, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\management_buttons.rs".to_string(),
            regions: (16, 16, 0.00),
            functions: (6, 6, 0.00),
            lines: (6, 6, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\meme_management.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\memes.rs".to_string(),
            regions: (137, 137, 0.00),
            functions: (51, 51, 0.00),
            lines: (102, 102, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\meta_meme_operations.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\notification.rs".to_string(),
            regions: (92, 92, 0.00),
            functions: (20, 20, 0.00),
            lines: (62, 62, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\page_not_found.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\query_accounts.rs".to_string(),
            regions: (42, 42, 0.00),
            functions: (5, 5, 0.00),
            lines: (24, 24, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\receive_sol.rs".to_string(),
            regions: (25, 25, 0.00),
            functions: (6, 6, 0.00),
            lines: (15, 15, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\send_sol.rs".to_string(),
            regions: (46, 46, 0.00),
            functions: (7, 7, 0.00),
            lines: (30, 30, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\styling_and_emojis.rs".to_string(),
            regions: (1, 1, 0.00),
            functions: (1, 1, 0.00),
            lines: (1, 1, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\transaction_buttons.rs".to_string(),
            regions: (10, 10, 0.00),
            functions: (4, 4, 0.00),
            lines: (4, 4, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\wikidata_memes\\mod.rs".to_string(),
            regions: (77, 77, 0.00),
            functions: (11, 11, 0.00),
            lines: (81, 81, 0.00),
            branches: (0, 0, 0.00),
        },
        CoverageEntry {
            filename: "views\\workflow_memes\\mod.rs".to_string(),
            regions: (58, 58, 0.00),
            functions: (4, 4, 0.00),
            lines: (140, 140, 0.00),
            branches: (0, 0, 0.00),
        },
    ]
}
