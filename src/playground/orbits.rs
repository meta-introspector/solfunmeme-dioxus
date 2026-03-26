use crate::stubs::motion::prelude::*;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use nalgebra::{vector, SVector};
type Vector8<T> = SVector<T, 8>;
type State = Vector8<f64>;
use emojis;

// To modify the Dioxus application so that each node in the ThemeOrbitalNetwork has its own mass, position, and velocity, and moves along its own unique 4D orbit (projected to 2D), we’ll extend the existing system. This aligns with your previous request to give each object its own orbit with dynamic calculations (from June 24, 2025, 10:03). We’ll:
// Update ThemeNode: Add mass, position, and velocity fields to support individual 4D orbits.
// Modify Simulation: Simulate each node’s 4D orbit independently, using its mass in the gravitational force
// F = -\frac{k}{r^3} \hat{r}
// .
// Update ThemeOrbitalNetwork: Render each node’s 2D-projected orbit as an SVG path, with nodes moving along their paths using dioxus-motion for animation.
// Adjust CSS: Ensure styles support multiple orbit paths and animated nodes.
// Update get_orbit_nodes: Incorporate mass and initial conditions for each node.
// Keep Test: Update the test module to verify multiple orbits.
// This will replace the single 4D orbit with four independent orbits (one per node), each with unique dynamics based on mass, position, and velocity, while maintaining the interactive node behavior (click, hover, spark effects).
// Step-by-Step Implementation
// 1. Update ThemeNode Struct
// Add fields for mass, initial position, and initial velocity in 4D space. Since nodes move along their orbits, we’ll track their current position dynamically in the simulation.
// rust
#[derive(Clone, Debug, PartialEq)]
struct ThemeNode {
    emoji: String,
    color: String,
    mass: f64,                  // Mass affecting gravitational force
    initial_position: [f64; 4], // Initial (x, y, z, w)
    initial_velocity: [f64; 4], // Initial (vx, vy, vz, vw)
}
// 2. Update get_orbit_nodes
// Modify get_orbit_nodes to include mass and initial conditions for each of the four nodes. We’ll assign distinct masses and slightly varied initial positions/velocities to create unique orbits.
// rust

fn get_orbit_nodes(count: usize) -> Vec<ThemeNode> {
    let base_nodes = vec![
        ThemeNode {
            emoji: emojis::get_by_shortcode("rocket").unwrap().to_string(),
            color: "rgba(255, 0, 0, 0.8)".to_string(),
            mass: 1.0,
            initial_position: [1.0, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.5, 0.3, 0.2],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("scroll").unwrap().to_string(),
            color: "rgba(255, 255, 0, 0.8)".to_string(),
            mass: 1.5,
            initial_position: [1.2, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.45, 0.25, 0.15],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("mag").unwrap().to_string(),
            color: "rgba(0, 255, 255, 0.8)".to_string(),
            mass: 0.8,
            initial_position: [0.9, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.55, 0.35, 0.25],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("speech_balloon")
                .unwrap()
                .to_string(),
            color: "rgba(255, 0, 255, 0.8)".to_string(),
            mass: 1.2,
            initial_position: [1.1, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.48, 0.28, 0.18],
        },
    ];
    base_nodes.into_iter().take(count).collect()
}

fn get_orbit_nodes2(count: usize) -> Vec<ThemeNode> {
    let base_nodes = vec![
        ThemeNode {
            emoji: "🚀".to_string(),
            color: "rgba(255, 0, 0, 0.8)".to_string(),
            mass: 1.0,
            initial_position: [1.0, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.5, 0.3, 0.2],
        },
        ThemeNode {
            emoji: "📜".to_string(),
            color: "rgba(255, 255, 0, 0.8)".to_string(),
            mass: 1.5,
            initial_position: [1.2, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.45, 0.25, 0.15],
        },
        ThemeNode {
            emoji: "🔍".to_string(),
            color: "rgba(0, 255, 255, 0.8)".to_string(),
            mass: 0.8,
            initial_position: [0.9, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.55, 0.35, 0.25],
        },
        ThemeNode {
            emoji: "💬".to_string(),
            color: "rgba(255, 0, 255, 0.8)".to_string(),
            mass: 1.2,
            initial_position: [1.1, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.48, 0.28, 0.18],
        },
    ];
    base_nodes.into_iter().take(count).collect()
}
// Masses: Vary between 0.8 and 1.5 to affect orbit shapes (higher mass tightens orbits for the same force).
// Positions: Start near ( (1, 0, 0, 0) ) with slight offsets to avoid identical paths.
// Velocities: Adjusted to ensure bound orbits (avoiding collapse or escape).
// 3. Update Simulation Logic
// Each node needs its own 4D orbit, computed using its mass in the force equation. We’ll reuse the existing simulate_orbit but modify it to return the full trajectory and support per-node mass. We’ll also precompute orbits to avoid browser performance issues.
// Update the simulation functions (place near the top of the file):
// rust

fn derivatives(state: &State, _t: f64, k: f64, m: f64) -> State {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let w = state[3];
    let vx = state[4];
    let vy = state[5];
    let vz = state[6];
    let vw = state[7];
    let r = (x * x + y * y + z * z + w * w).sqrt();
    let r_cubed = r.powi(3);
    let factor = -k / (m * r_cubed); // Mass affects acceleration
    let ax = factor * x;
    let ay = factor * y;
    let az = factor * z;
    let aw = factor * w;
    //State::new(vx, vy, vz, vw, ax, ay, az, aw)
    vector![vx, vy, vz, vw, ax, ay, az, aw]
}

fn rk4_step(state: &State, t: f64, dt: f64, k: f64, m: f64) -> State {
    let k1 = derivatives(state, t, k, m);
    let k2 = derivatives(&(state + 0.5 * dt * k1), t + 0.5 * dt, k, m);
    let k3 = derivatives(&(state + 0.5 * dt * k2), t + 0.5 * dt, k, m);
    let k4 = derivatives(&(state + dt * k3), t + dt, k, m);
    state + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}

fn simulate_orbit(
    t_span: (f64, f64),
    n_steps: usize,
    initial_state: State,
    k: f64,
    m: f64,
) -> Vec<(f64, f64)> {
    let (t0, tf) = t_span;
    let dt = (tf - t0) / (n_steps as f64);
    let mut points = Vec::with_capacity(n_steps + 1);
    let mut state = initial_state;
    let mut t = t0;
    points.push((state[0], state[1]));
    for _ in 0..n_steps {
        state = rk4_step(&state, t, dt, k, m);
        t += dt;
        points.push((state[0], state[1]));
    }
    points
}
// Change: The m parameter (mass) now directly affects the acceleration in derivatives, scaling the force.
// 4. Update ThemeOrbitalNetwork
// Each node’s orbit is computed separately, rendered as an SVG path, and the node is animated along its path using dioxus-motion. We’ll normalize all orbits to fit the 800x800 SVG canvas and animate nodes to move cyclically.
// rust
#[component]
pub fn ThemeOrbitalNetwork2() -> Element {
    let selected_node = use_signal(|| None::<usize>);
    let k = 1.0; // Force constant
    let t_span = (0.0, 10.0);
    let n_steps = 1000;
    let nodes = get_orbit_nodes(4);

    // Compute orbits for each node
    let orbits: Vec<Vec<(f64, f64)>> = nodes
        .iter()
        .map(|node| {
            let initial_state = vector![
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

    // Animation state for each node
    //let mut positions = use_signal(|| vec![0usize; nodes.len()]); // Current point index
    // use_effect(move || {
    //     spawn(async move {
    //         //loop {
    //             //TimeoutFuture::new(16).await; // ~60 FPS
    // 		//let newpos =
    //             //positions.write();
    // 		// fixme!
    //     //}
    //     });
    // });
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

    rsx! {
        div { class: "orbit_4d_container",
            svg {
                width: "800",
                height: "800",
                for (i, path_data) in paths.iter().enumerate() {
                    path {
                        key: "{i}",
                        class: "orbit_4d_path",
                        d: "{path_data}",
                        style: format!("stroke: {}", nodes[i].color.replace("0.8", "0.5"))
                    }
                }
            }
            // for (i, node) in nodes.iter().enumerate() {
            //     let p = orbits[i][positions.read()[i]];
        // 	let x =p[0];
        // 	let y=p[1];
            //     let px = x * scale + offset_x;
            //     let py = y * scale + offset_y;
            //     ThemeNodeComponent {
            //         key: "{i}",
            //         node: node.clone(),
            //         position_angle: 0.0,
            //         radius: 0.0,
            //         style: format!("left: {}px; top: {}px;", px - 25.0, py - 25.0),
            //         on_click: move |_| {
            //             handle_node_click(&mut selected_node, i);
            //         }
            //     }
            // }
        }
    }
}

// use dioxus::prelude::*;
// use crate::stubs::motion::prelude::*;
// use gloo_timers::future::TimeoutFuture;
// use rand::Rng;
// use std::time::Duration;
// use wasm_bindgen::JsCast;
// use web_sys::{console, window};
// use easer::functions::Easing;
// use crate::stubs::motion::animations::utils::LoopMode;
// use nalgebra::{Vector4, Vector8};
// use emojis;

// ... (Keep simulation functions, STYLES, ThemeNode struct, get_orbit_nodes unchanged)

fn generate_path_elements(paths: &[String], nodes: &[ThemeNode]) -> Vec<Element> {
    // Changed from Vec<impl Into<Element>>
    paths
        .iter()
        .enumerate()
        .map(|(i, path_data)| {
            rsx! {
                path {
                    key: "{i}",
                    class: "orbit_4d_path",
                    d: "{path_data}",
                    style: format!("stroke: {}", nodes[i].color.replace("0.8", "0.5"))
                }
            }
        })
        .collect()
}

// Generate SVG path elements for orbits
fn generate_path_elements2(paths: &[String], nodes: &[ThemeNode]) -> Vec<impl Into<Element>> {
    paths
        .iter()
        .enumerate()
        .map(|(i, path_data)| {
            rsx! {
                path {
                    key: "{i}",
                    class: "orbit_4d_path",
                    d: "{path_data}",
                    style: format!("stroke: {}", nodes[i].color.replace("0.8", "0.5"))
                }
            }
        })
        .collect()
}
fn generate_node_elements(
    nodes: &[ThemeNode],
    orbits: &[Vec<(f64, f64)>],
    positions: Signal<Vec<usize>>,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    selected_node: &mut Signal<Option<usize>>,
) -> Vec<Element> {
    // Changed from Vec<impl Into<Element>>
    nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let (x, y) = orbits[i][positions.read()[i]];
            let px = x * scale + offset_x;
            let py = y * scale + offset_y;
            let node_clone = node.clone();
            let selected_node_clone = selected_node.clone();
            rsx! {
                div {
                    "node {i}"
                }
            }
        })
        .collect()
}

// Generate ThemeNodeComponent elements
fn generate_node_elements2(
    nodes: &[ThemeNode],
    orbits: &[Vec<(f64, f64)>],
    positions: Signal<Vec<usize>>,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    selected_node: &mut Signal<Option<usize>>,
) -> Vec<impl Into<Element>> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let (x, y) = orbits[i][positions.read()[i]];
            let px = x * scale + offset_x;
            let py = y * scale + offset_y;
            let node_clone = node.clone();
            let selected_node_clone = selected_node.clone();
            rsx! {
            div {
                "node {i}"
            }
                    // ThemeNodeComponent {
                    //     key: "{i}",
                    //     node: node_clone,
                    //     position_angle: 0.0,
                    //     radius: 0.0,
                    //     style: format!("left: {}px; top: {}px;", px - 25.0, py - 25.0),
                    //     on_click: move |_| {
                    //         handle_node_click(&mut selected_node_clone, i);
                    //     }
                    // }
                }
        })
        .collect()
}

#[component]
pub fn ThemeOrbitalNetwork4() -> Element {
    let mut selected_node = use_signal(|| None::<usize>);
    let k = 1.0;
    let t_span = (0.0, 10.0);
    let n_steps = 1000;
    let nodes = get_orbit_nodes(4);

    let orbits: Vec<Vec<(f64, f64)>> = nodes
        .iter()
        .map(|node| {
            let initial_state = vector![
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
                let mut pos1 = positions.write();
                pos1.iter_mut().for_each(|p| *p = (*p + 1) % n_steps);
            }
        });
    });

    let gen0 = generate_path_elements(&paths, &nodes).into_iter();
    let gen1 = generate_node_elements(
        &nodes,
        &orbits,
        positions,
        scale,
        offset_x,
        offset_y,
        &mut selected_node,
    )
    .into_iter();
    rsx! {
        div { class: "orbit_4d_container", svg { width: "800", height: "800", {gen0}  } {gen1}  }
    }
}

// Changes:
// Computes a separate orbit for each node using its mass and initial conditions.
// Normalizes all orbits together to ensure they fit the SVG canvas.
// Renders each orbit as a unique SVG path, colored to match the node.
// Animates nodes along their orbits by incrementing their position index every 16ms (~60 FPS).
// Nodes are positioned at their current orbit point, updated reactively.
// 5. Update CSS
// Adjust the .orbit_4d_path style to support per-orbit coloring and ensure nodes remain visible. The existing .orbit_node style is fine, but we’ll tweak the path for clarity.
// In STYLES, update:
// rust
const STYLES2: &str = r#"
        .orbit_4d_container {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 1;
            width: 800px;
            height: 800px;
        }

        .orbit_4d_path {
            stroke-width: 1.5;
            fill: none;
            filter: drop-shadow(0 0 5px rgba(255, 255, 255, 0.3));
        }

        .orbit_node {
            position: absolute;
            width: 40px;
            height: 40px;
            border: 2px solid #fff;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.2em;
            cursor: pointer;
            transition: all 0.3s ease;
            animation: pulse 2s ease-in-out infinite;
            z-index: 10;
        }

        @keyframes pulse {
            0%, 100% { transform: scale(1); }
            50% { transform: scale(1.15); }
        }

        .orbit_node:hover {
            transform: scale(1.4) !important;
            box-shadow: 0 0 20px currentColor;
            z-index: 15;
        }
"#;

// Changes:
// Thinner paths (stroke-width: 1.5) to avoid clutter with multiple orbits.
// Subtle glow effect for paths.
// Smaller nodes (40px) for better proportionality.
// Adjusted hover and pulse effects for smoother visuals.
// 6. Update Test Module
// Update the test to verify multiple orbits with different masses, ensuring the simulation handles per-node dynamics correctly.
// rust
#[cfg(test)]
mod tests2 {
    use super::{get_orbit_nodes, simulate_orbit, State};
    use csv::Writer;
    use std::fs::File;

    #[test]
    fn test_4d_orbit_simulation() {
        let k = 1.0;
        let t_span = (0.0, 10.0);
        let n_steps = 1000;
        let nodes = get_orbit_nodes(4);

        let orbits: Vec<Vec<(f64, f64)>> = nodes
            .iter()
            .map(|node| {
                let initial_state = State::from_row_slice(&[
                    node.initial_position[0],
                    node.initial_position[1],
                    node.initial_position[2],
                    node.initial_position[3],
                    node.initial_velocity[0],
                    node.initial_velocity[1],
                    node.initial_velocity[2],
                    node.initial_velocity[3],
                ]);
                simulate_orbit(t_span, n_steps, initial_state, k, node.mass)
            })
            .collect();

        let file = File::create("test_orbits_2d_projection.csv").unwrap();
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(["node", "t", "x", "y"]).unwrap();
        let dt = (t_span.1 - t_span.0) / (n_steps as f64);
        for (i, orbit) in orbits.iter().enumerate() {
            for (j, &(x, y)) in orbit.iter().enumerate() {
                let t = t_span.0 + (j as f64) * dt;
                wtr.write_record([i.to_string(), t.to_string(), x.to_string(), y.to_string()])
                    .unwrap();
            }
        }
        wtr.flush().unwrap();

        assert_eq!(orbits.len(), 4, "Should have 4 orbits");
        assert!(
            orbits.iter().all(|o| o.len() == n_steps + 1),
            "Each orbit should have n_steps + 1 points"
        );
        assert!(
            orbits
                .iter()
                .flatten()
                .all(|&(x, y)| x.is_finite() && y.is_finite()),
            "Orbit points should be finite"
        );
    }
}
// 7. Notes on Visualization
// Orbits: Each node follows a unique 2D-projected 4D orbit, influenced by its mass. Higher-mass nodes (e.g., 📜 at 1.5) have tighter orbits for the same force, while lower-mass nodes (e.g., 🔍 at 0.8) have wider paths.
// Animation: Nodes move smoothly along their orbits, cycling every ~10 seconds (1000 steps at 60 FPS). The paths are static but colored to match each node.
// Interactivity: Nodes retain click (spark) and hover (scale) effects, with handle_node_click logging selections.
// Performance: Precomputing orbits and using SVG paths ensures browser efficiency. Animation is lightweight via dioxus-motion.
// 8. Optional Enhancements
// Dynamic Forces: Add node-node interactions (e.g., mutual gravity) for more complex dynamics.
// Orbit Switching: Allow nodes to swap orbits on click, using reactive signals.
// 3D Projection: Extend to 3D visualization using WebGL (requires significant changes).
// Benchmarking: Add a separate bench test for orbit calculations, as previously discussed (June 24, 2025).
// 9. Full Updated Code (Relevant Sections)
// For brevity, I’ll show only the changed sections. Replace these in your existing code, keeping unchanged components (SolFunNiceApp, BoostCore, etc.) and utility functions.
// rust

//type State = Vector8<f64>;

fn derivatives2(state: &State, _t: f64, k: f64, m: f64) -> State {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let w = state[3];
    let vx = state[4];
    let vy = state[5];
    let vz = state[6];
    let vw = state[7];
    let r = (x * x + y * y + z * z + w * w).sqrt();
    let r_cubed = r.powi(3);
    let factor = -k / (m * r_cubed);
    let ax = factor * x;
    let ay = factor * y;
    let az = factor * z;
    let aw = factor * w;
    vector![vx, vy, vz, vw, ax, ay, az, aw]
}

fn rk4_step2(state: &State, t: f64, dt: f64, k: f64, m: f64) -> State {
    let k1 = derivatives(state, t, k, m);
    let k2 = derivatives(&(state + 0.5 * dt * k1), t + 0.5 * dt, k, m);
    let k3 = derivatives(&(state + 0.5 * dt * k2), t + 0.5 * dt, k, m);
    let k4 = derivatives(&(state + dt * k3), t + dt, k, m);
    state + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}

fn simulate_orbit2(
    t_span: (f64, f64),
    n_steps: usize,
    initial_state: State,
    k: f64,
    m: f64,
) -> Vec<(f64, f64)> {
    let (t0, tf) = t_span;
    let dt = (tf - t0) / (n_steps as f64);
    let mut points = Vec::with_capacity(n_steps + 1);
    let mut state = initial_state;
    let mut t = t0;
    points.push((state[0], state[1]));
    for _ in 0..n_steps {
        state = rk4_step(&state, t, dt, k, m);
        t += dt;
        points.push((state[0], state[1]));
    }
    points
}

const STYLES: &str = r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            background: linear-gradient(45deg, #000000, #1a0033, #330066, #6600ff);
            background-size: 400% 400%;
            animation: gradientShift 8s ease infinite;
            color: #fff;
            font-family: 'Courier New', monospace;
            overflow: hidden;
            height: 100vh;
        }

        @keyframes gradientShift {
            0% { background-position: 0% 50%; }
            50% { background-position: 100% 50%; }
            100% { background-position: 0% 50%; }
        }

        .zos-container {
            position: relative;
            width: 100vw;
            height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .pump-core {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 20;
            text-align: center;
            background: rgba(102, 0, 255, 0.1);
            border: 3px solid #00ff00;
            border-radius: 50px;
            padding: 40px;
            backdrop-filter: blur(15px);
            box-shadow: 0 0 50px rgba(0, 255, 0, 0.5);
            animation: hyperPump 2s ease-in-out infinite;
        }

        @keyframes hyperPump {
            0%, 100% { 
                transform: translate(-50%, -50%) scale(1) rotate(0deg);
                box-shadow: 0 0 50px rgba(0, 255, 0, 0.5);
            }
            25% { 
                transform: translate(-50%, -50%) scale(1.1) rotate(2deg);
                box-shadow: 0 0 80px rgba(255, 0, 255, 0.7);
            }
            50% { 
                transform: translate(-50%, -50%) scale(1.2) rotate(0deg);
                box-shadow: 0 0 100px rgba(0, 255, 255, 0.8);
            }
            75% { 
                transform: translate(-50%, -50%) scale(1.1) rotate(-2deg);
                box-shadow: 0 0 80px rgba(255, 255, 0, 0.7);
            }
        }

        .title {
            font-size: 2.5em;
            color: #00ff00;
            margin-bottom: 10px;
            text-shadow: 0 0 20px #00ff00;
            animation: textGlow 3s ease-in-out infinite alternate;
        }

        @keyframes textGlow {
            from { text-shadow: 0 0 20px #00ff00; }
            to { text-shadow: 0 0 40px #00ff00, 0 0 60px #00ff00; }
        }

        .subtitle {
            font-size: 1.2em;
            color: #ff00ff;
            margin-bottom: 20px;
            animation: rainbow 4s linear infinite;
        }

        @keyframes rainbow {
            0% { color: #ff00ff; }
            16% { color: #ff0080; }
            33% { color: #ff0000; }
            50% { color: #ff8000; }
            66% { color: #ffff00; }
            83% { color: #00ff00; }
            100% { color: #00ffff; }
        }

        .emoji-engine {
            font-size: 3em;
            margin: 20px 0;
            animation: bounce 1.5s ease-in-out infinite;
        }

        @keyframes bounce {
            0%, 100% { transform: translateY(0px) scale(1); }
            50% { transform: translateY(-20px) scale(1.1); }
        }

        .orbit_4d_container {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 1;
            width: 800px;
            height: 800px;
        }

        .orbit_4d_path {
            stroke-width: 1.5;
            fill: none;
            filter: drop-shadow(0 0 5px rgba(255, 255, 255, 0.3));
        }

        .orbit_node {
            position: absolute;
            width: 40px;
            height: 40px;
            border: 2px solid #fff;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.2em;
            cursor: pointer;
            transition: all 0.3s ease;
            animation: pulse 2s ease-in-out infinite;
            z-index: 10;
        }

        @keyframes pulse {
            0%, 100% { transform: scale(1); }
            50% { transform: scale(1.15); }
        }

        .orbit_node:hover {
            transform: scale(1.4) !important;
            box-shadow: 0 0 20px currentColor;
            z-index: 15;
        }

        .features-panel {
            position: absolute;
            top: 20px;
            left: 20px;
            background: rgba(0, 0, 0, 0.8);
            padding: 20px;
            border-radius: 15px;
            border: 2px solid #00ff00;
            max-width: 350px;
            z-index: 15;
            animation: slideIn 1s ease-out;
        }

        @keyframes slideIn {
            from { transform: translateX(-100%); opacity: 0; }
            to { transform: translateX(0); opacity: 1; }
        }

        .feature {
            display: flex;
            align-items: center;
            margin: 10px 0;
            font-size: 0.9em;
            animation: fadeIn 2s ease-in;
        }

        .feature-emoji {
            font-size: 1.5em;
            margin-right: 10px;
            animation: wiggle 3s ease-in-out infinite;
        }

        @keyframes wiggle {
            0%, 100% { transform: rotate(0deg); }
            25% { transform: rotate(5deg); }
            75% { transform: rotate(-5deg); }
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }

        .consensus-panel {
            position: absolute;
            bottom: 20px;
            left: 20px;
            background: rgba(102, 0, 255, 0.2);
            padding: 20px;
            border-radius: 15px;
            border: 2px solid #ff00ff;
            max-width: 300px;
            z-index: 15;
        }

        .workflow-panel {
            position: absolute;
            top: 20px;
            right: 20px;
            background: rgba(0, 0, 0, 0.8);
            padding: 20px;
            border-radius: 15px;
            border: 2px solid #ffff00;
            max-width: 300px;
            z-index: 15;
        }

        .workflow-step {
            margin: 10px 0;
            padding: 10px;
            background: rgba(255, 255, 0, 0.1);
            border-radius: 8px;
            font-size: 0.85em;
        }

        .pump-metrics {
            position: absolute;
            bottom: 20px;
            right: 20px;
            background: rgba(0, 255, 0, 0.1);
            padding: 20px;
            border-radius: 15px;
            border: 2px solid #00ffff;
            z-index: 15;
        }

        .metric {
            margin: 8px 0;
            font-size: 0.9em;
        }

        .metric-value {
            color: #00ff00;
            font-weight: bold;
            animation: flicker 2s linear infinite;
        }

        @keyframes flicker {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.7; }
        }

        .particle {
            position: absolute;
            width: 16px;
            height: 16px;
            background: #00ff00;
            border-radius: 50%;
            animation: float 6s linear infinite;
        }

        @keyframes float {
            0% { transform: translateY(100vh) scale(0); opacity: 0; }
            10% { opacity: 1; }
            90% { opacity: 1; }
            100% { transform: translateY(-100px) scale(1); opacity: 0; }
        }

        .glow-text {
            text-shadow: 0 0 10px currentColor;
        }
"#;

#[derive(Clone, Debug, PartialEq)]
struct ThemeNode2 {
    emoji: String,
    color: String,
    mass: f64,
    initial_position: [f64; 4],
    initial_velocity: [f64; 4],
}

fn get_orbit_nodes3(count: usize) -> Vec<ThemeNode> {
    let base_nodes = vec![
        ThemeNode {
            emoji: "🚀".to_string(),
            color: "rgba(255, 0, 0, 0.8)".to_string(),
            mass: 1.0,
            initial_position: [1.0, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.5, 0.3, 0.2],
        },
        ThemeNode {
            emoji: "📜".to_string(),
            color: "rgba(255, 255, 0, 0.8)".to_string(),
            mass: 1.5,
            initial_position: [1.2, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.45, 0.25, 0.15],
        },
        ThemeNode {
            emoji: "🔍".to_string(),
            color: "rgba(0, 255, 255, 0.8)".to_string(),
            mass: 0.8,
            initial_position: [0.9, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.55, 0.35, 0.25],
        },
        ThemeNode {
            emoji: "💬".to_string(),
            color: "rgba(255, 0, 255, 0.8)".to_string(),
            mass: 1.2,
            initial_position: [1.1, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.48, 0.28, 0.18],
        },
    ];
    base_nodes.into_iter().take(count).collect()
}

#[component]
pub fn ThemeOrbitalNetwork3() -> Element {
    let selected_node = use_signal(|| None::<usize>);
    let k = 1.0;
    let t_span = (0.0, 10.0);
    let n_steps = 1000;
    let nodes = get_orbit_nodes(4);

    let orbits: Vec<Vec<(f64, f64)>> = nodes
        .iter()
        .map(|node| {
            let initial_state = vector![
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

    rsx! {
        div { class: "orbit_4d_container",
            svg {
                width: "800",
                height: "800",
                for (i, path_data) in paths.iter().enumerate() {
                    path {
                        key: "{i}",
                        class: "orbit_4d_path",
                        d: "{path_data}",
                        style: format!("stroke: {}", nodes[i].color.replace("0.8", "0.5"))
                    }
                }
            }
              //for (i, node) in nodes.iter().enumerate() {
            //     let (x, y) = orbits[i][positions.read()[i]];
            //     let px = x * scale + offset_x;
            //     let py = y * scale + offset_y;
            //     ThemeNodeComponent {
            //         key: "{i}",
            //         node: node.clone(),
            //         position_angle: 0.0,
            //         radius: 0.0,
            //         style: format!("left: {}px; top: {}px;", px - 25.0, py - 25.0),
            //         on_click: move |_| {
            //             handle_node_click(&mut selected_node, i);
            //         }
            //     }
            // }
        }
    }
}

// ... (Keep ThemeNodeComponent, other components, and utility functions unchanged)

#[cfg(test)]
mod tests {
    use super::{get_orbit_nodes, simulate_orbit, State};
    use csv::Writer;
    use std::fs::File;

    #[test]
    fn test_4d_orbit_simulation() {
        let k = 1.0;
        let t_span = (0.0, 10.0);
        let n_steps = 1000;
        let nodes = get_orbit_nodes(4);

        let orbits: Vec<Vec<(f64, f64)>> = nodes
            .iter()
            .map(|node| {
                let initial_state = State::from_row_slice(&[
                    node.initial_position[0],
                    node.initial_position[1],
                    node.initial_position[2],
                    node.initial_position[3],
                    node.initial_velocity[0],
                    node.initial_velocity[1],
                    node.initial_velocity[2],
                    node.initial_velocity[3],
                ]);
                simulate_orbit(t_span, n_steps, initial_state, k, node.mass)
            })
            .collect();

        let file = File::create("test_orbits_2d_projection.csv").unwrap();
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(["node", "t", "x", "y"]).unwrap();
        let dt = (t_span.1 - t_span.0) / (n_steps as f64);
        for (i, orbit) in orbits.iter().enumerate() {
            for (j, &(x, y)) in orbit.iter().enumerate() {
                let t = t_span.0 + (j as f64) * dt;
                wtr.write_record([i.to_string(), t.to_string(), x.to_string(), y.to_string()])
                    .unwrap();
            }
        }
        wtr.flush().unwrap();

        assert_eq!(orbits.len(), 4, "Should have 4 orbits");
        assert!(
            orbits
                .iter()
                .all(|o: &Vec<(f64, f64)>| o.len() == n_steps + 1),
            "Each orbit should have n_steps + 1 points"
        );
        assert!(
            orbits
                .iter()
                .flatten()
                .all(|&(x, y): &(f64, f64)| x.is_finite() && y.is_finite()),
            "Orbit points should be finite"
        );
    }
}
// Running the App
// Update Cargo.toml with dependencies (unchanged from previous responses).
// Run dioxus serve to launch the app.
// Verify that four nodes (🚀, 📜, 🔍, 💬) move along distinct, color-coded orbit paths, with unique trajectories based on their masses.
// If you want to add node interactions, adjust orbit parameters, or benchmark the calculations separately, let me know!
