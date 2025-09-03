use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use gloo_timers::future::TimeoutFuture;
use orbital_sim_lib::{get_orbit_nodes, simulate_orbit, ThemeNode};
use std::collections::HashMap;
use emoji_matrix_lib::{parse_summary_total, parse_summary_root, rollup_emoji_matrix};
use core_data_lib::{EmojiMatrix, EmojiMatrixEntry, EmojiCount};
use rand::Rng;

// Function to convert EmojiMatrix data into ThemeNodes
fn create_theme_nodes_from_emoji_matrix(emoji_matrix: EmojiMatrix) -> Vec<ThemeNode> {
    let mut theme_nodes = Vec::new();
    let mut rng = rand::thread_rng();

    // Simple color palette for variety
    let colors = [
        "rgba(255, 0, 0, 0.8)",    // Red
        "rgba(0, 255, 0, 0.8)",    // Green
        "rgba(0, 0, 255, 0.8)",    // Blue
        "rgba(255, 255, 0, 0.8)",  // Yellow
        "rgba(0, 255, 255, 0.8)",  // Cyan
        "rgba(255, 0, 255, 0.8)",  // Magenta
        "rgba(255, 128, 0, 0.8)",  // Orange
        "rgba(128, 0, 255, 0.8)",  // Purple
    ];

    for entry in emoji_matrix.entries {
        for emoji_count in entry.emoji_counts {
            // Map emoji to ThemeNode.emoji
            let emoji = emoji_count.emoji;

            // Assign color deterministically or randomly
            let color = colors[rng.gen_range(0..colors.len())].to_string();

            // Derive mass from emoji count (e.g., count / 100.0)
            let mass = (emoji_count.count as f64 / 100.0).max(0.1); // Ensure mass is not zero

            // Generate initial position and velocity (simple example)
            let initial_position = [
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
            ];
            let initial_velocity = [
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
            ];

            theme_nodes.push(ThemeNode {
                emoji,
                color,
                mass,
                initial_position,
                initial_velocity,
            });
        }
    }
    theme_nodes
}

// Styles for the orbital network visualization
const STYLES: &str = r#"
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }
    body {
        background-color: #1a1a1a;
        color: #f0f0f0;
        font-family: sans-serif;
        overflow: hidden;
    }
    .orbit_4d_container {
        position: relative;
        width: 800px;
        height: 800px;
        border: 1px solid #333;
        background-color: #000;
        margin: 50px auto;
        overflow: hidden;
    }
    .orbit_4d_path {
        fill: none;
        stroke-width: 2;
        stroke-linecap: round;
        stroke-linejoin: round;
        opacity: 0.7;
    }
    .node_component {
        position: absolute;
        width: 50px;
        height: 50px;
        border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 30px;
        cursor: pointer;
        transition: transform 0.1s ease-out;
        transform-origin: center;
        box-shadow: 0 0 10px rgba(255, 255, 255, 0.3);
    }
    .node_component:hover {
        transform: scale(1.2);
        box-shadow: 0 0 20px rgba(255, 255, 255, 0.6);
    }
    .node_component.selected {
        border: 3px solid white;
        box-shadow: 0 0 25px white;
    }
    .node_component span {
        text-shadow: 0 0 10px currentColor;
    }
"#;

// Component for displaying individual nodes
#[component]
fn ThemeNodeComponent(
    node: ThemeNode,
    position_angle: f64, // Not used in 4D projection, but kept for consistency
    radius: f64,         // Not used in 4D projection, but kept for consistency
    style: String,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "node_component",
            style: "{style}",
            onclick: move |evt| on_click.call(evt),
            span { "{node.emoji}" }
        }
    }
}

// Handles node clicks (for logging or future interactivity)
fn handle_node_click(selected_node: &mut Signal<Option<usize>>, index: usize) {
    *selected_node.write() = Some(index);
    log::info!("Node {} clicked!", index);
}

// Generates SVG path elements for orbits
fn generate_path_elements<'a>(
    paths: &'a Vec<String>,
    nodes: &'a Vec<ThemeNode>,
) -> impl Iterator<Item = Element> + 'a {
    paths.iter().enumerate().map(move |(i, path_data)| {
        rsx! {
            path {
                key: "{i}",
                class: "orbit_4d_path",
                d: "{path_data}",
                style: format!("stroke: {}", nodes[i].color.replace("0.8", "0.5"))
            }
        }
    })
}

// Generates node elements for the visualization
fn generate_node_elements<'a>(
    nodes: &'a Vec<ThemeNode>,
    orbits: &'a Vec<Vec<(f64, f64)>>,
    positions: Signal<Vec<usize>>,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    selected_node: Signal<Option<usize>>,
) -> impl Iterator<Item = Element> + 'a {
    nodes.iter().enumerate().map(move |(i, node)| {
        let (x, y) = orbits[i][positions.read()[i]];
        let px = x * scale + offset_x;
        let py = y * scale + offset_y;
        let is_selected = selected_node.read().map_or(false, |s_idx| s_idx == i);
        rsx! {
            ThemeNodeComponent {
                key: "{i}",
                node: node.clone(),
                position_angle: 0.0, // Placeholder
                radius: 0.0, // Placeholder
                style: format!(
                    "left: {}px; top: {}px; background-color: {}; {}",
                    px - 25.0,
                    py - 25.0,
                    node.color,
                    if is_selected { "border: 3px solid white;" } else { "" }
                ),
                on_click: move |_| {
                    let mut selected_node_clone = selected_node.clone();
                    handle_node_click(&mut selected_node_clone, i);
                }
            }
        }
    })
}

// Main component for the 4D orbital network visualization
#[component]
pub fn ThemeOrbitalNetwork() -> Element {
    let mut selected_node = use_signal(|| None::<usize>);
    let k = 1.0; // Force constant
    let t_span = (0.0, 10.0);
    let n_steps = 1000;
    let mut nodes = use_signal(|| Vec::<ThemeNode>::new());

    use_future(move || async move {
        let total_matrix = parse_summary_total();
        let root_matrix = parse_summary_root();
        let mut combined_entries = total_matrix.entries;
        combined_entries.extend(root_matrix.entries);
        let final_matrix = rollup_emoji_matrix(EmojiMatrix { entries: combined_entries });
        nodes.set(create_theme_nodes_from_emoji_matrix(final_matrix));
    });

    // Compute orbits for each node
    let orbits: Vec<Vec<(f64, f64)>> = nodes
        .iter()
        .map(|node| {
            let initial_state = nalgebra::vector![
                node.initial_position[0],
                node.initial_position[1],
                node.initial_position[2],
                node.initial_position[3],
                node.initial_velocity[0],
                node.initial_velocity[1],
                node.initial_velocity[2],
                node.initial_velocity[3],
            ];
            simulate_orbit(t_span, n_steps, initial_state, k, node.mass)
        })
        .collect();

    // Normalize all orbits to fit 800x800 SVG
    let all_points: Vec<(f64, f64)> = orbits.iter().flatten().copied().collect();
    let (min_x, max_x) = all_points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(x, _)| {
            (min.min(x), max.max(x))
        });
    let (min_y, max_y) = all_points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)| {
            (min.min(y), max.max(y))
        });
    let x_range = max_x - min_x;
    let y_range = max_y - min_y;
    let scale = 700.0 / x_range.max(y_range);
    let offset_x = 400.0 - scale * (min_x + x_range / 2.0);
    let offset_y = 400.0 - scale * (min_y + y_range / 2.0);

    // Generate path data for each orbit
    let paths: Vec<String> = orbits
        .iter()
        .map(|orbit| {
            orbit
                .iter()
                .enumerate()
                .map(|(i, &(x, y))| {
                    let px = x * scale + offset_x;
                    let py = y * scale + offset_y;
                    if i == 0 {
                        format!("M {} {}", px, py)
                    } else {
                        format!("L {} {}", px, py)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    let mut positions = use_signal(|| vec![0usize; nodes.len()]);
    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(16).await;
                let mut pos = positions.write();
                pos.iter_mut().for_each(|p| *p = (*p + 1) % n_steps);
            }
        });
    });

    let read_nodes = nodes.read();
    let gen0 = generate_path_elements(&paths, &read_nodes).into_iter();
    let gen1 = generate_node_elements(
        &read_nodes,
        &orbits,
        positions,
        scale,
        offset_x,
        offset_y,
        selected_node,
    )
    .into_iter();
    rsx! {
        style { {STYLES} }
        div { class: "orbit_4d_container",
            svg { width: "800", height: "800", {gen0} }
            {gen1}
        }
    }
}
