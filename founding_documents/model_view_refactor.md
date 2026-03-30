# Model-View Refactoring for Dioxus Application

## Overview

Splitting the code into a **model** (business logic and data processing) and **view** (UI rendering) is a great architectural choice for your Dioxus application. This separation enhances testability, maintainability, and reusability. We'll create a `model` module to handle the 768D BERT embedding reduction to 8D and manage the `ThemeNode` data, and a `view` module to render the UI using this data. We'll also add a benchmark to test the model's performance and ensure the view uses the same logic.

Given the current date and time (10:54 AM PDT, Monday, June 23, 2025), we'll assume this is a fresh start based on our previous discussions, using `dioxus-motion` 0.3.1 (as it worked for you) and integrating the 8D hypersphere concept with BERT embeddings.

## Project Structure

- **src/model.rs**: Contains the data structures, embedding reduction logic, and node generation.
- **src/view.rs**: Contains the Dioxus components for rendering.
- **src/main.rs**: Orchestrates the app, loading the model and rendering the view.
- **benches/model_bench.rs**: Benchmarks the model's performance.

## Step 1: Model Module

The model will reduce 768D BERT embeddings to 8D using PCA and generate `ThemeNode` instances.

```rust
// src/model.rs
use linfa::prelude::*;
use linfa_preprocessing::PrincipalComponentAnalysis;
use ndarray::{array, Array2};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThemeNode {
    pub emoji: String,
    pub color: String,
    pub dim_values: [f64; 8],
}

pub fn reduce_bert_to_8d(embeddings: Array2<f64>) -> Vec<ThemeNode> {
    let pca = PrincipalComponentAnalysis::fit(&embeddings).unwrap().with_components(8);
    let reduced_8d = pca.transform(&embeddings);

    let emojis = vec!["🚀", "📜", "🔍", "💬", "🔀", "💡", "💭", "🔑"];
    let colors = vec![
        "rgba(255, 0, 0, 0.8)", "rgba(255, 255, 0, 0.8)", "rgba(0, 255, 255, 0.8)",
        "rgba(255, 0, 255, 0.8)", "rgba(0, 255, 0, 0.8)", "rgba(255, 128, 0, 0.8)",
        "rgba(128, 255, 0, 0.8)", "rgba(255, 0, 128, 0.8)",
    ];

    reduced_8d.outer_iter()
        .zip(emojis.iter().cycle().zip(colors.iter().cycle()))
        .map(|(row, (emoji, color))| ThemeNode {
            emoji: emoji.to_string(),
            color: color.to_string(),
            dim_values: row.to_vec().try_into().unwrap(),
        })
        .collect()
}

pub fn load_or_generate_nodes() -> Vec<ThemeNode> {
    // Check if precomputed data exists
    if let Ok(data) = fs::read_to_string("src/bert_8d.json") {
        if let Ok(nodes) = serde_json::from_str::<Vec<ThemeNode>>(&data) {
            return nodes;
        }
    }

    // Mock embeddings for testing (replace with actual BERT embeddings)
    let embeddings = array![
        [0.1; 768],
        [0.2; 768],
        [0.3; 768],
        [0.4; 768],
        [0.5; 768],
        [0.6; 768],
        [0.7; 768],
        [0.8; 768],
    ];
    let nodes = reduce_bert_to_8d(embeddings);
    let json = serde_json::to_string_pretty(&nodes).unwrap();
    fs::write("src/bert_8d.json", json).expect("Failed to write JSON");
    nodes
}
```

**Dependencies**: Add to `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.6", features = ["web"] }
dioxus-motion = "0.3.1"
linfa = "0.7"
linfa-preprocessing = "0.7"
ndarray = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2.87"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

## Step 2: View Module

The view handles the Dioxus components, using the model's data.

```rust
// src/view.rs
use dioxus::prelude::*;
use dioxus_motion::{use_motion, Transform, AnimationConfig, AnimationMode, LoopMode};
use std::f64::consts::PI;
use crate::model::ThemeNode;

#[component]
pub fn ThemeNodeComponent(cx: Scope, node: ThemeNode, position_angle: f64, radius: f64, on_click: EventHandler<()>) -> Element {
    let rad = position_angle.to_radians();
    let center_x = radius as f64;
    let center_y = radius as f64;
    let x = center_x + (radius * rad.cos());
    let y = center_y + (radius * rad.sin());

    let hue = (node.dim_values[2] * 360.0).rem_euclid(360.0);
    let size = 40.0 + (node.dim_values[3] * 20.0);
    let opacity = (node.dim_values[4] + 0.5).clamp(0.5, 1.0);
    let rotation_speed = node.dim_values[5] * 2.0;
    let brightness = (node.dim_values[6] * 0.5 + 0.5).clamp(0.5, 1.0);
    let phase_offset = node.dim_values[7] * 2.0 * PI;

    let mut node_rotation = use_motion(cx, 0.0);
    use_effect(cx, (), |_| {
        let speed = rotation_speed;
        spawn(async move {
            loop {
                node_rotation.animate_to(
                    360.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis((1000.0 / speed.max(0.1)).round() as u64),
                        easing: easer::functions::Linear::ease_in_out,
                    }))
                    .loop_mode(Some(LoopMode::Infinite))
                    .build(),
                );
                TimeoutFuture::new(1000).await;
            }
        });
    });

    let rotation = node_rotation.get_value() + phase_offset.to_degrees();
    let style = format!(
        "position: absolute; width: {}px; height: {}px; background: hsla({}, 80%, {}%, {}); border: 2px solid #fff; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 1.5em; cursor: pointer; transition: all 0.3s ease; transform: translate({}px, {}px) rotate({}deg); animation: pulse 2s ease-in-out infinite;",
        size, size, hue, brightness * 100.0, opacity * 100.0, x - size / 2.0, y - size / 2.0, rotation
    );

    cx.render(rsx! {
        div {
            class: "meme-node",
            style: "{style}",
            onclick: move |_| on_click.call(()),
            "{node.emoji}"
        }
    })
}

#[component]
pub fn ThemeOrbit(cx: Scope, radius: u32, duration: u32, reverse: bool, nodes: Vec<ThemeNode>, on_node_click: EventHandler<usize>) -> Element {
    let mut orbit_motion = use_motion(cx, Transform::identity());
    use_effect(cx, (reverse, duration), |(reverse, duration)| {
        let rotation_direction = if reverse { -360.0 } else { 360.0 };
        orbit_motion.animate_to(
            Transform::new(0.0, 0.0, 1.0, rotation_direction),
            AnimationConfig {
                mode: AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(duration as u64),
                    easing: easer::functions::Linear::ease_in_out,
                }),
                loop_mode: Some(LoopMode::Infinite),
                ..Default::default()
            },
        );
    });

    let radius = radius;
    let radius2 = radius / 2;
    let rotation = orbit_motion.get_value().rotation.unwrap_or(0.0);
    let orbit_style = format!(
        "position: absolute; width: {radius}px; height: {radius}px; margin: -{radius2}px 0 0 -{radius2}px; border: 2px solid rgba(0, 255, 0, 0.3); border-radius: 50%; animation: spin {}s linear infinite; transform: rotate({}deg);",
        duration, rotation
    );

    if nodes.is_empty() {
        return cx.render(rsx! { div { class: "orbit", style: "{orbit_style}" } });
    }

    cx.render(rsx! {
        div {
            class: "orbit",
            style: "{orbit_style}",
            for (index, node) in nodes.iter().enumerate() {
                ThemeNodeComponent {
                    key: "{index}",
                    node: node.clone(),
                    position_angle: (index as f64 * 360.0) / nodes.len() as f64,
                    radius: radius as f64,
                    on_click: move |_| on_node_click.call(index)
                }
            }
        }
    })
}

#[component]
pub fn ThemeOrbitalNetwork(cx: Scope) -> Element {
    let nodes = use_context_provider(cx, || crate::model::load_or_generate_nodes());
    let selected_node = use_signal(cx, || None::<usize>);

    cx.render(rsx! {
        div {
            class: "meme-orbits",
            style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); z-index: 1;",
            ThemeOrbit {
                radius: 300,
                duration: 8,
                reverse: false,
                nodes: nodes.read().clone(),
                on_node_click: move |node_id| selected_node.set(Some(node_id))
            }
            ThemeOrbit {
                radius: 500,
                duration: 12,
                reverse: true,
                nodes: nodes.read().clone(),
                on_node_click: move |node_id| selected_node.set(Some(node_id))
            }
            ThemeOrbit {
                radius: 700,
                duration: 16,
                reverse: false,
                nodes: nodes.read().clone(),
                on_node_click: move |node_id| selected_node.set(Some(node_id))
            }
        }
    })
}
```

## Step 3: Main Module

Orchestrate the app and provide the model data.

```rust
// src/main.rs
use dioxus::prelude::*;
mod model;
mod view;

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    use_context_provider(cx, || model::load_or_generate_nodes());
    cx.render(rsx! {
        head {
            style { "
                @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
                @keyframes pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.2); } }
            " }
        }
        body {
            view::ThemeOrbitalNetwork {}
        }
    })
}
```

## Step 4: Benchmark

Create a benchmark to test the model's `reduce_bert_to_8d` function.

```rust
// benches/model_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::array;
use solfunnice::model;

fn benchmark_bert_reduction(c: &mut Criterion) {
    let embeddings = array![
        [0.1; 768],
        [0.2; 768],
        [0.3; 768],
        [0.4; 768],
        [0.5; 768],
        [0.6; 768],
        [0.7; 768],
        [0.8; 768],
    ];
    c.bench_function("reduce_bert_to_8d", |b| b.iter(|| model::reduce_bert_to_8d(black_box(embeddings.clone()))));
}

criterion_group!(benches, benchmark_bert_reduction);
criterion_main!(benches);
```

**Run Benchmark**: `cargo bench -- --nocapture` to see performance results (e.g., time to reduce embeddings).

## Integration and Testing

- **Build**: Run `cargo build` to generate `bert_8d.json`, then `dx build --platform web` and `dx serve --platform web`.
- **Test**: The view will use the same model logic, ensuring consistency between benchmarks and UI.
- **Extend**: Add more sophisticated BERT embeddings and additional 8D dimensions for richer visualizations.

This architecture provides a clean separation of concerns, making the codebase more maintainable and testable while preserving the visual appeal of the orbital network. 