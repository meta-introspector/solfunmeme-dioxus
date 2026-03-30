// // Cargo.toml
// /*
// [package]
// name = "solfunmeme-zos"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// dioxus = { version = "0.5", features = ["web", "router"] }
// dioxus-motion = "0.1"
// web-sys = "0.3"
// wasm-bindgen = "0.2"
// gloo-timers = "0.3"
// rand = "0.8"
// */
// use dioxus::prelude::*;
// use dioxus_motion::*;
// use gloo_timers::future::TimeoutFuture;
// use rand::Rng;
// use std::time::Duration;
// use web_sys::{console, window};

// // Main application entry point
// fn main() {
//     dioxus::launch(App);
// }

// // Root component
// #[component]
// fn App() -> Element {
//     rsx! {
//        // document::Link { rel: "stylesheet", href: asset!("./assets/style.css") }
//         div {
//             class: "zos-container",
//             //ZeroOntologySystem {}
// 	    "FIXME"
//         }
//     }
// }

// // // Core ZOS system component
// // #[component]
// // fn ZeroOntologySystem() -> Element {
// //     let mut pump_active = use_signal(|| false);
// //     let mut Engaging_score = use_signal(|| 847.3);
// //     let mut meme_velocity = use_signal(|| "∞ pumps/sec".to_string());
// //     let mut consensus = use_signal(|| 99.7);
// //     let mut hype_level = use_signal(|| "MAXIMUM".to_string());
// //     let mut current_emoji_sequence = use_signal(|| 0);

// //     // Initialize periodic updates
// //     use_effect(move || {
// //         spawn(async move {
// //             loop {
// //                 TimeoutFuture::new(3_000).await;
// //                 update_metrics(&mut Engaging_score, &mut consensus);
// //             }
// //         });
// //     });

// //     // Initialize emoji rotation
// //     use_effect(move || {
// //         spawn(async move {
// //             loop {
// //                 TimeoutFuture::new(2_000).await;
// //                 rotate_emoji_sequence(&mut current_emoji_sequence);
// //             }
// //         });
// //     });

// //     rsx! {
// //         // Floating particles system
// //         ParticleSystem {}

// //         // Orbital meme network
// //         MemeOrbitalNetwork {}

// //         // Central pump core
// //         PumpCore {
// //             pump_active: pump_active.read().clone(),
// //             current_emoji_sequence: current_emoji_sequence.read().clone(),
// //             on_pump: move |_| trigger_hyper_pump(&mut pump_active)
// //         }

// //         // UI Panels
// //         FeaturesPanel {}
// //         ConsensusPanel {
// //             Engaging_score: Engaging_score.read().clone(),
// //             meme_velocity: meme_velocity.read().clone(),
// //             consensus: consensus.read().clone(),
// //             hype_level: hype_level.read().clone()
// //         }
// //         WorkflowPanel {}
// //         PumpMetricsPanel {}

// //         // Global event handlers
// //         GlobalEventHandlers {
// //             on_space_pressed: move |_| trigger_mega_pump(&mut pump_active)
// //         }
// //     }
// // }

// // Particle system component
// // #[component]
// // fn ParticleSystem() -> Element {
// //     let mut particles = use_signal(|| Vec::<ParticleState>::new());

// //     // Spawn particles continuously
// //     use_effect(move || {
// //         spawn(async move {
// //             loop {
// //                 TimeoutFuture::new(500).await;
// //                 spawn_particle(&mut particles);
// //             }
// //         });
// //     });

// //     rsx! {
// //         div { id: "particles",
// //             for particle in particles.read().iter() {
// //                 Particle {
// //                     key: "{particle.id}",
// //                     x: particle.x,
// //                     y: particle.y,
// //                     color: particle.color.clone(),
// //                     duration: particle.duration
// //                 }
// //             }
// //         }
// //     }
// // }

// // // Individual particle component
// // #[component]
// // fn Particle(x: f64, y: f64, color: String, duration: u32) -> Element {
// //     let motion = use_motion_state(|| MotionState::new());

// //     use_effect(move || {
// //         motion.animate_to(
// //             Motion::new()
// //                 .duration(Duration::from_millis(duration as u64))
// //                 .translate_y(-100.0)
// //                 .scale(1.0)
// //                 .opacity(0.0)
// //         );
// //     });

// //     rsx! {
// //         div {
// //             class: "particle",
// //             style: "left: {x}%; background-color: {color};",
// //             Motion { state: motion }
// //         }
// //     }
// // }

// // Meme orbital network component
// // #[component]
// // fn MemeOrbitalNetwork() -> Element {
// //     let mut selected_node = use_signal(|| None::<usize>);

// //     rsx! {
// //         div { class: "meme-orbits",
// //             MemeOrbit {
// //                 radius: 300,
// //                 duration: 8,
// //                 reverse: false,
// //                 nodes: get_orbit_nodes(0),
// //                 on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
// //             }
// //             MemeOrbit {
// //                 radius: 500,
// //                 duration: 12,
// //                 reverse: true,
// //                 nodes: get_orbit_nodes(1),
// //                 on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
// //             }
// //             MemeOrbit {
// //                 radius: 700,
// //                 duration: 16,
// //                 reverse: false,
// //                 nodes: get_orbit_nodes(2),
// //                 on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
// //             }
// //         }
// //     }
// // }

// // // Individual orbit component
// // #[component]
// // fn MemeOrbit(
// //     radius: u32,
// //     duration: u32,
// //     reverse: bool,
// //     nodes: Vec<MemeNode>,
// //     on_node_click: EventHandler<usize>
// // ) -> Element {
// //     let orbit_motion = use_motion_state(|| MotionState::new());

// //     use_effect(move || {
// //         let rotation_direction = if reverse { -360.0 } else { 360.0 };
// //         orbit_motion.animate_loop(
// //             Motion::new()
// //                 .duration(Duration::from_secs(duration as u64))
// //                 .rotate(rotation_direction)
// //         );
// //     });

// //     rsx! {
// //         div {
// //             class: "orbit",
// //             style: "width: {radius}px; height: {radius}px; margin: -{radius/2}px 0 0 -{radius/2}px;",
// //             Motion { state: orbit_motion },
// //             for (index, node) in nodes.iter().enumerate() {
// //                 MemeNodeComponent {
// //                     key: "{index}",
// //                     node: node.clone(),
// //                     position_angle: (index as f64 * 360.0) / nodes.len() as f64,
// //                     radius: radius as f64 / 2.0,
// //                     on_click: move |_| on_node_click.call(index)
// //                 }
// //             }
// //         }
// //     }
// // }

// // // Individual meme node component
// // #[component]
// // fn MemeNodeComponent(
// //     node: MemeNode,
// //     position_angle: f64,
// //     radius: f64,
// //     on_click: EventHandler<()>
// // ) -> Element {
// //     let mut is_exploding = use_signal(|| false);
// //     let node_motion = use_motion_state(|| MotionState::new());

// //     // Calculate position
// //     let x = radius * position_angle.to_radians().cos();
// //     let y = radius * position_angle.to_radians().sin();

// //     rsx! {
// //         div {
// //             class: "meme-node",
// //             style: "
// //                 left: {x + radius}px;
// //                 top: {y + radius}px;
// //                 background-color: {node.color};
// //             ",
// //             Motion { state: node_motion },
// //             onclick: move |_| {
// //                 trigger_node_explosion(&mut is_exploding, &node_motion);
// //                 on_click.call(());
// //             },
// //             onmouseenter: move |_| hover_node_enter(&node_motion),
// //             onmouseleave: move |_| hover_node_exit(&node_motion),
// //             "{node.emoji}"

// //             if is_exploding.read().clone() {
// //                 NodeExplosion {
// //                     emoji: node.emoji.clone(),
// //                     on_complete: move |_| is_exploding.set(false)
// //                 }
// //             }
// //         }
// //     }
// // }

// // Node explosion effect
// #[component]
// fn NodeExplosion(emoji: String, on_complete: EventHandler<()>) -> Element {
//     let mut explosions = use_signal(|| Vec::<ExplosionParticle>::new());

//     use_effect(move || {
//         // Create explosion particles
//         let mut particles = Vec::new();
//         for i in 0..10 {
//             particles.push(ExplosionParticle {
//                 id: i,
//                 x: 0.0,
//                 y: 0.0,
//                 velocity_x: (rand::thread_rng().gen::<f64>() - 0.5) * 200.0,
//                 velocity_y: (rand::thread_rng().gen::<f64>() - 0.5) * 200.0,
//                 scale: 1.0 + rand::thread_rng().gen::<f64>() * 2.0,
//             });
//         }
//         explosions.set(particles);

//         // Clean up after animation
//         spawn(async move {
//             TimeoutFuture::new(1_000).await;
//             on_complete.call(());
//         });
//     });

//     rsx! {
//         div { class: "explosion-container",
//             for particle in explosions.read().iter() {
//                 ExplosionParticleComponent {
//                     key: "{particle.id}",
//                     emoji: emoji.clone(),
//                     particle: particle.clone()
//                 }
//             }
//         }
//     }
// }

// // Individual explosion particle
// // #[component]
// // fn ExplosionParticleComponent(emoji: String, particle: ExplosionParticle) -> Element {
// //     let motion = use_motion_state(|| MotionState::new());

// //     use_effect(move || {
// //         motion.animate_to(
// //             Motion::new()
// //                 .duration(Duration::from_millis(1000))
// //                 .translate_x(particle.velocity_x)
// //                 .translate_y(particle.velocity_y)
// //                 .scale(particle.scale * 3.0)
// //                 .opacity(0.0)
// //         );
// //     });

// //     rsx! {
// //         div {
// //             class: "explosion-particle",
// //             style: "position: absolute; font-size: 2em; pointer-events: none;",
// //             Motion { state: motion },
// //             "{emoji}"
// //         }
// //     }
// // }

// // // Central pump core component
// // #[component]
// // fn PumpCore(
// //     pump_active: bool,
// //     current_emoji_sequence: usize,
// //     on_pump: EventHandler<()>
// // ) -> Element {
// //     let core_motion = use_motion_state(|| MotionState::new());
// //     let emoji_sequences = get_emoji_sequences();

// //     // Animate pump effect when active
// //     use_effect(move || {
// //         if pump_active {
// //             core_motion.animate_to(
// //                 Motion::new()
// //                     .duration(Duration::from_millis(500))
// //                     .scale(1.3)
// //                     .then(Motion::new()
// //                         .duration(Duration::from_millis(500))
// //                         .scale(1.0)
// //                     )
// //             );
// //         }
// //     });

// //     rsx! {
// //         div {
// //             class: "pump-core",
// //             Motion { state: core_motion },
// //             onclick: move |_| on_pump.call(()),

// //             h1 { class: "title", "SOLFUNMEME" }
// //             h2 { class: "subtitle", "Zero Ontology System" }

// //             div {
// //                 class: "emoji-engine",
// //                 "{emoji_sequences[current_emoji_sequence % emoji_sequences.len()]}"
// //             }

// //             div { class: "core-description",
// //                 div { class: "glow-text", "Meta-Meme Pump Protocol" }
// //                 div { class: "core-tags", "Self-Introspective • Recursive • Engaging" }
// //             }
// //         }
// //     }
// // }

// // Features panel component
// #[component]
// fn FeaturesPanel() -> Element {
//     let features = get_system_features();

//     rsx! {
//         div { class: "features-panel",
//             h3 { "Key Features" }
//             for feature in features {
//                 FeatureItem {
//                     emoji: feature.emoji,
//                     text: feature.text
//                 }
//             }
//         }
//     }
// }

// // // Individual feature item
// // #[component]
// // fn FeatureItem(emoji: String, text: String) -> Element {
// //     let emoji_motion = use_motion_state(|| MotionState::new());

// //     use_effect(move || {
// //         // Wiggle animation
// //         emoji_motion.animate_loop(
// //             Motion::new()
// //                 .duration(Duration::from_millis(3000))
// //                 .rotate(5.0)
// //                 .then(Motion::new()
// //                     .duration(Duration::from_millis(3000))
// //                     .rotate(-5.0)
// //                 )
// //         );
// //     });

// //     rsx! {
// //         div { class: "feature",
// //             span {
// //                 class: "feature-emoji",
// //                 Motion { state: emoji_motion },
// //                 "{emoji}"
// //             }
// //             span { "{text}" }
// //         }
// //     }
// // }

// // Consensus panel component
// #[component]
// fn ConsensusPanel(
//     Engaging_score: f64,
//     meme_velocity: String,
//     consensus: f64,
//     hype_level: String
// ) -> Element {
//     rsx! {
//         div { class: "consensus-panel",
//             h3 { "Paxos Consensus" }

//             MetricDisplay {
//                 label: "Engaging Score:",
//                 value: format!("{:.1}K", Engaging_score),
//                 color: "#00ff00"
//             }
//             MetricDisplay {
//                 label: "Meme Velocity:",
//                 value: meme_velocity,
//                 color: "#ffff00"
//             }
//             MetricDisplay {
//                 label: "Consensus:",
//                 value: format!("{:.1}% PUMP", consensus),
//                 color: "#00ffff"
//             }
//             MetricDisplay {
//                 label: "Hype Level:",
//                 value: hype_level,
//                 color: "#ff00ff"
//             }
//         }
//     }
// }

// // Workflow panel component
// #[component]
// fn WorkflowPanel() -> Element {
//     let workflow_steps = get_workflow_steps();

//     rsx! {
//         div { class: "workflow-panel",
//             h3 { "How It Works" }
//             for step in workflow_steps {
//                 WorkflowStep {
//                     number: step.number,
//                     title: step.title,
//                     description: step.description,
//                     color: step.color
//                 }
//             }
//         }
//     }
// }

// // Individual workflow step
// #[component]
// fn WorkflowStep(number: String, title: String, description: String, color: String) -> Element {
//     rsx! {
//         div { class: "workflow-step",
//             span { style: "color: {color};", "{number}" }
//             " {title}"
//             br {}
//             small { "{description}" }
//         }
//     }
// }

// // Pump metrics panel
// #[component]
// fn PumpMetricsPanel() -> Element {
//     let mut metrics = use_signal(|| get_initial_metrics());

//     // Update metrics periodically
//     use_effect(move || {
//         spawn(async move {
//             loop {
//                 TimeoutFuture::new(2_000).await;
//                 update_pump_metrics(&mut metrics);
//             }
//         });
//     });

//     rsx! {
//         div { class: "pump-metrics",
//             h3 { "Live Metrics" }
//             for metric in metrics.read().iter() {
//                 MetricDisplay {
//                     label: metric.label.clone(),
//                     value: metric.value.clone(),
//                     color: "#00ffff"
//                 }
//             }
//         }
//     }
// }

// // Reusable metric display component
// #[component]
// fn MetricDisplay(label: String, value: String, color: String) -> Element {
//     rsx! {
//         div { class: "metric",
//             span { "{label}" }
//             " "
//             span {
//                 class: "metric-value",
//                 style: "color: {color};",
//                 "{value}"
//             }
//         }
//     }
// }

// // Global event handlers
// #[component]
// fn GlobalEventHandlers(on_space_pressed: EventHandler<()>) -> Element {
//     use_effect(move || {
//         let window = window().unwrap();
//         let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//             if event.code() == "Space" {
//                 event.prevent_default();
//                 on_space_pressed.call(());
//             }
//         }) as Box<dyn FnMut(_)>);

//         window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
//         closure.forget();
//     });

//     rsx! { div { hidden: true } }
// }

// // Data structures
// #[derive(Clone, Debug, PartialEq)]
// struct MemeNode {
//     emoji: String,
//     color: String,
// }

// #[derive(Clone, Debug)]
// struct ParticleState {
//     id: usize,
//     x: f64,
//     y: f64,
//     color: String,
//     duration: u32,
// }

// #[derive(Clone, Debug)]
// struct ExplosionParticle {
//     id: usize,
//     x: f64,
//     y: f64,
//     velocity_x: f64,
//     velocity_y: f64,
//     scale: f64,
// }

// #[derive(Clone, Debug)]
// struct SystemFeature {
//     emoji: String,
//     text: String,
// }

// #[derive(Clone, Debug)]
// struct WorkflowStepData {
//     number: String,
//     title: String,
//     description: String,
//     color: String,
// }

// #[derive(Clone, Debug)]
// struct MetricData {
//     label: String,
//     value: String,
// }

// // Utility functions
// fn get_orbit_nodes(orbit_index: usize) -> Vec<MemeNode> {
//     match orbit_index {
//         0 => vec![
//             MemeNode { emoji: "🚀".to_string(), color: "rgba(255, 0, 0, 0.8)".to_string() },
//             MemeNode { emoji: "📜".to_string(), color: "rgba(255, 255, 0, 0.8)".to_string() },
//             MemeNode { emoji: "🔍".to_string(), color: "rgba(0, 255, 255, 0.8)".to_string() },
//             MemeNode { emoji: "💬".to_string(), color: "rgba(255, 0, 255, 0.8)".to_string() },
//         ],
//         1 => vec![
//             MemeNode { emoji: "🔀".to_string(), color: "rgba(0, 255, 0, 0.8)".to_string() },
//             MemeNode { emoji: "💡".to_string(), color: "rgba(255, 128, 0, 0.8)".to_string() },
//             MemeNode { emoji: "💭".to_string(), color: "rgba(128, 255, 0, 0.8)".to_string() },
//             MemeNode { emoji: "🔑".to_string(), color: "rgba(255, 0, 128, 0.8)".to_string() },
//         ],
//         2 => vec![
//             MemeNode { emoji: "🤖".to_string(), color: "rgba(128, 0, 255, 0.8)".to_string() },
//             MemeNode { emoji: "🌐".to_string(), color: "rgba(0, 128, 255, 0.8)".to_string() },
//             MemeNode { emoji: "📊".to_string(), color: "rgba(255, 255, 128, 0.8)".to_string() },
//             MemeNode { emoji: "🔗".to_string(), color: "rgba(128, 255, 255, 0.8)".to_string() },
//         ],
//         _ => vec![]
//     }
// }

// fn get_emoji_sequences() -> Vec<String> {
//     vec![
//         "🚀📜🔍💬🧠".to_string(),
//         "🔀💡💭🔑".to_string(),
//         "🤖🌐📊🔗".to_string(),
//         "🧩🔗🌱".to_string(),
//         "💎🚀💰🔥".to_string(),
//         "🌙📈💫⚡".to_string(),
//         "🎯🎪🎨🎭".to_string(),
//         "🔮✨🌟💫".to_string(),
//     ]
// }

// fn get_system_features() -> Vec<SystemFeature> {
//     vec![
//         SystemFeature { emoji: "🚀".to_string(), text: "Self-Introspective Meme Engine".to_string() },
//         SystemFeature { emoji: "🔀".to_string(), text: "Paxos Meme Consensus".to_string() },
//         SystemFeature { emoji: "📈".to_string(), text: "Hyper-Pump Mechanism".to_string() },
//         SystemFeature { emoji: "📜".to_string(), text: "Semantic Compression".to_string() },
//         SystemFeature { emoji: "🔗".to_string(), text: "Immutable Meme-State".to_string() },
//         SystemFeature { emoji: "🌱".to_string(), text: "Meme Mining & Propagation".to_string() },
//     ]
// }

// fn get_workflow_steps() -> Vec<WorkflowStepData> {
//     vec![
//         WorkflowStepData {
//             number: "1️⃣".to_string(),
//             title: "ZOS Interaction".to_string(),
//             description: "Interactive memetic encoding/decoding".to_string(),
//             color: "#00ff00".to_string(),
//         },
//         WorkflowStepData {
//             number: "2️⃣".to_string(),
//             title: "Paxos Consensus".to_string(),
//             description: "Community shapes meme evolution".to_string(),
//             color: "#ff00ff".to_string(),
//         },
//         WorkflowStepData {
//             number: "3️⃣".to_string(),
//             title: "Self-Improvment".to_string(),
//             description: "Evolves via hype & engagement".to_string(),
//             color: "#00ffff".to_string(),
//         },
//         WorkflowStepData {
//             number: "4️⃣".to_string(),
//             title: "Engaging Propagation".to_string(),
//             description: "Holders influence narrative shifts".to_string(),
//             color: "#ffff00".to_string(),
//         },
//     ]
// }

// fn get_initial_metrics() -> Vec<MetricData> {
//     vec![
//         MetricData { label: "Meme Strength:".to_string(), value: "ULTRA RARE".to_string() },
//         MetricData { label: "Pump Factor:".to_string(), value: "∞x".to_string() },
//         MetricData { label: "Engaging Coefficient:".to_string(), value: "1.847".to_string() },
//         MetricData { label: "Meta Level:".to_string(), value: "RECURSIVE".to_string() },
//         MetricData { label: "Status:".to_string(), value: "PUMPING".to_string() },
//     ]
// }

// // Event handler functions
// fn update_metrics(Engaging_score: &mut Signal<f64>, consensus: &mut Signal<f64>) {
//     let mut rng = rand::thread_rng();
//     let current_score = Engaging_score.read().clone();
//     Engaging_score.set(current_score + rng.gen_range(-5.0..10.0));

//     let current_consensus = consensus.read().clone();
//     consensus.set((current_consensus + rng.gen_range(-0.5..0.5)).clamp(95.0, 100.0));
// }

// fn rotate_emoji_sequence(current_sequence: &mut Signal<usize>) {
//     let sequences = get_emoji_sequences();
//     let current = current_sequence.read().clone();
//     current_sequence.set((current + 1) % sequences.len());
// }

// fn spawn_particle(particles: &mut Signal<Vec<ParticleState>>) {
//     let mut rng = rand::thread_rng();
//     let colors = ["#00ff00", "#ff00ff", "#00ffff", "#ffff00", "#ff0080"];

//     let new_particle = ParticleState {
//         id: rng.gen(),
//         x: rng.gen_range(0.0..100.0),
//         y: 100.0,
//         color: colors[rng.gen_range(0..colors.len())].to_string(),
//         duration: rng.gen_range(4000..8000),
//     };

//     let mut current_particles = particles.read().clone();
//     current_particles.push(new_particle);

//     // Keep only recent particles
//     if current_particles.len() > 20 {
//         current_particles.remove(0);
//     }

//     particles.set(current_particles);
// }

// fn handle_node_click(selected_node: &mut Signal<Option<usize>>, node_id: usize) {
//     selected_node.set(Some(node_id));
//     console::log_1(&format!("Node {} clicked!", node_id).into());
// }

// // fn trigger_node_explosion(is_exploding: &mut Signal<bool>, node_motion: &MotionState) {
// //     is_exploding.set(true);

// //     // Trigger node shake effect
// //     node_motion.animate_to(
// //         Motion::new()
// //             .duration(Duration::from_millis(100))
// //             .scale(1.3)
// //             .then(Motion::new()
// //                 .duration(Duration::from_millis(100))
// //                 .scale(1.0)
// //             )
// //     );
// // }

// // fn hover_node_enter(node_motion: &MotionState) {
// //     node_motion.animate_to(
// //         Motion::new()
// //             .duration(Duration::from_millis(200))
// //             .scale(1.2)
// //     );
// // }

// // fn hover_node_exit(node_motion: &MotionState) {
// //     node_motion.animate_to(
// //         Motion::new()
// //             .duration(Duration::from_millis(200))
// //             .scale(1.0)
// //     );
// // }

// fn trigger_hyper_pump(pump_active: &mut Signal<bool>) {
//     pump_active.set(true);

//     // Reset pump state after animation
//     let mut pump_active_clone = pump_active.clone();
//     spawn(async move {
//         TimeoutFuture::new(1_000).await;
//         pump_active_clone.set(false);
//     });
// }

// fn trigger_mega_pump(pump_active: &mut Signal<bool>) {
//     pump_active.set(true);
//     console::log_1(&"🚀 MEGA PUMP ACTIVATED! 🚀".into());

//     // Create screen flash effect
//     if let Some(body) = web_sys::window()
//         .and_then(|w| w.document())
//         .and_then(|d| d.body())
//     {
//         let _ = body.style().set_property("background",
//             "linear-gradient(45deg, #ff0000, #ff00ff, #00ffff, #00ff00)");

//         // Reset after mega pump
//         let body_clone = body.clone();
//         spawn(async move {
//             TimeoutFuture::new(2_000).await;
//             let _ = body_clone.style().set_property("background",
//                 "linear-gradient(45deg, #000000, #1a0033, #330066, #6600ff)");
//         });
//     }

//     // Reset pump state
//     let mut pump_active_clone = pump_active.clone();
//     spawn(async move {
//         TimeoutFuture::new(2_000).await;
//         pump_active_clone.set(false);
//     });
// }

// fn update_pump_metrics(metrics: &mut Signal<Vec<MetricData>>) {
//     let mut rng = rand::thread_rng();
//     let mut current_metrics = metrics.read().clone();

//     // Update Engaging coefficient
//     for metric in &mut current_metrics {
//         if metric.label == "Engaging Coefficient:" {
//             let new_value = 1.0 + rng.gen_range(0.0..1.0);
//             metric.value = format!("{:.3}", new_value);
//         }
//     }

//     metrics.set(current_metrics);
// }
