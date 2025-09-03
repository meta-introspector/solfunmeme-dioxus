use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use gloo_timers::future::TimeoutFuture;
use rand::Rng;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{console, window};
//use dioxus_motion::AnimationStep;
use crate::playground::orbits::*;
use dioxus_motion::animations::utils::LoopMode;
use easer::functions::Easing;

// Welcome to the SOLFUNTHEME Zero Ontology System (ZOS)!
// This is the entry point for our joyful, interactive theme-powered application.
//fn main() {
//    dioxus::launch(App);
//}
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

        .meme-orbits {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            z-index: 1;
        }

        .orbit {
            position: absolute;
            background: transparent;
            border: 2px solid rgba(0, 255, 0, 0.3);
            border-radius: 50%;
            animation: spin 10s linear infinite;
        }

        .orbit-1 {
            width: 300px;
            height: 300px;
            margin: -150px 0 0 -150px;
            animation-duration: 8s;
        }

        .orbit-2 {
            width: 500px;
            height: 500px;
            margin: -250px 0 0 -250px;
            animation-duration: 12s;
            animation-direction: reverse;
        }

        .orbit-3 {
            width: 700px;
            height: 700px;
            margin: -350px 0 0 -350px;
            animation-duration: 16s;
        }

        @keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }

        .meme-node {
            position: absolute;
            width: 60px;
            height: 60px;
            background: rgba(255, 0, 255, 0.8);
            border: 2px solid #fff;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5em;
            cursor: pointer;
            transition: all 0.3s ease;
            animation: pulse 2s ease-in-out infinite;
        }

        @keyframes pulse {
            0%, 100% { transform: scale(1); }
            50% { transform: scale(1.2); }
        }

        .meme-node:hover {
            transform: scale(1.5) !important;
            box-shadow: 0 0 30px currentColor;
            z-index: 10;
        }

        .node-1 { top: -30px; left: 120px; background: rgba(255, 0, 0, 0.8); }
        .node-2 { top: 120px; left: 220px; background: rgba(255, 255, 0, 0.8); }
        .node-3 { top: 220px; left: 120px; background: rgba(0, 255, 255, 0.8); }
        .node-4 { top: 120px; left: 20px; background: rgba(255, 0, 255, 0.8); }

        .node-5 { top: -30px; left: 220px; background: rgba(0, 255, 0, 0.8); }
        .node-6 { top: 220px; left: 420px; background: rgba(255, 128, 0, 0.8); }
        .node-7 { top: 420px; left: 220px; background: rgba(128, 255, 0, 0.8); }
        .node-8 { top: 220px; left: 20px; background: rgba(255, 0, 128, 0.8); }

        .node-9 { top: -30px; left: 320px; background: rgba(128, 0, 255, 0.8); }
        .node-10 { top: 320px; left: 620px; background: rgba(0, 128, 255, 0.8); }
        .node-11 { top: 620px; left: 320px; background: rgba(255, 255, 128, 0.8); }
        .node-12 { top: 320px; left: 20px; background: rgba(128, 255, 255, 0.8); }

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

// The root component that sets up our cheerful application
#[component]
pub fn SolFunNiceApp() -> Element {
    rsx! {
        // Link our vibrant CSS styles
    //        document::Link { rel: "stylesheet", href: asset!("./assets/solfunmeme.css") }
    style { "{STYLES}" }
        div {
            class: "zos-container",
            // The core of our fun system
            ZeroOntologySystem {}
        }
    }
}

// The main ZOS component, bringing all the joy together
#[component]
fn ZeroOntologySystem() -> Element {
    // State signals for dynamic metrics
    let mut boost_active = use_signal(|| false);
    let mut fun_score = use_signal(|| 847.3);
    let theme_speed = use_signal(|| "∞ boosts/sec".to_string());
    let mut agreement = use_signal(|| 99.7);
    let excitement_level = use_signal(|| "MAXIMUM".to_string());
    let mut current_emoji_sequence = use_signal(|| 0);
    let mut particles = use_signal(|| Vec::<ParticleState>::new());
    // Keep metrics fresh with periodic updates
    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(3_000).await;
                update_metrics(&mut fun_score, &mut agreement);
            }
        });
    });

    // Rotate emoji sequences for extra sparkle
    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(2_000).await;
                rotate_emoji_sequence(&mut current_emoji_sequence);
            }
        });
    });

    rsx! {
            // Twinkling particle effects
            ParticleSystem { particles: particles.clone() }

            // Enchanting orbital theme network
    //        ThemeOrbitalNetwork {}
        ThemeOrbitalNetwork {}
    //	ThemeOrbitalNetwork3 {}
    //	ThemeOrbitalNetwork4 {}

            // The exciting boost core
            BoostCore {
                boost_active: boost_active.read().clone(),
                current_emoji_sequence: current_emoji_sequence.read().clone(),
                on_boost: move |_| trigger_super_boost(&mut boost_active)
            }

            // Friendly UI panels
            FeaturesPanel {}
            AgreementPanel {
                fun_score: fun_score.read().clone(),
                theme_speed: theme_speed.read().clone(),
                agreement: agreement.read().clone(),
                excitement_level: excitement_level.read().clone()
            }
            WorkflowPanel {}
            BoostMetricsPanel {}

            // Handle global keyboard events
            GlobalEventHandlers {
                on_space_pressed: move |_| trigger_mega_boost_with_particles(&mut boost_active, &mut particles)
            }
        }
}

// A sparkling particle system for visual delight
#[component]
fn ParticleSystem(particles: Signal<Vec<ParticleState>>) -> Element {
    let mut particles = use_signal(|| Vec::<ParticleState>::new());

    // Continuously spawn new particles
    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(500).await;
                spawn_particle(&mut particles);
            }
        });
    });

    rsx! {
        div { id: "particles",
            // Render each particle with a unique key
            for particle in particles.read().iter() {
                Particle {
                    key: "{particle.id}",
                    x: particle.x,
                    y: particle.y,
                    color: particle.color.clone(),
                    duration: particle.duration
                }
            }
        }
    }
}

// A single animated particle
#[component]
fn Particle(x: f64, y: f64, color: String, duration: u32) -> Element {
    let mut motion = use_motion(Transform::identity());
    let mut opacity = use_motion(1.0f32);

    // Animate the particle's journey
    use_effect(move || {
        motion.animate_to(
            Transform::new(0.0, -100.0, 1.0, 0.0),
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(duration as u64),
                easing: easer::functions::Linear::ease_in_out,
                ..Default::default()
            })),
        );
        opacity.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(duration as u64),
                easing: easer::functions::Linear::ease_in_out,
                ..Default::default()
            })),
        );
    });

    let opac = opacity.get_value();
    let transform = motion.get_value();
    let style = format!(
        "left: {x}%; background-color: {color}; opacity: {opac}; transform: translate({}px, {}px) scale({}) rotate({}deg);",
        transform.x,
        transform.y,
        transform.scale,
        transform.rotation
    );

    rsx! {
        div {
            class: "particle",
            style: style
        }
    }
}

// A captivating network of orbiting theme nodes
#[component]
fn ThemeOrbitalNetwork() -> Element {
    let mut selected_node = use_signal(|| None::<usize>);

    rsx! {
        div { class: "theme-orbits",
            // Three concentric orbits with different properties
            ThemeOrbit {
                radius: 300,
                duration: 8,
                reverse: false,
                nodes: get_orbit_nodes(0),
                on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
            }
            ThemeOrbit {
                radius: 500,
                duration: 12,
                reverse: true,
                nodes: get_orbit_nodes(1),
                on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
            }
            ThemeOrbit {
                radius: 700,
                duration: 16,
                reverse: false,
                nodes: get_orbit_nodes(2),
                on_node_click: move |node_id| handle_node_click(&mut selected_node, node_id)
            }
        }
    }
}

// A single orbiting path for theme nodes
#[component]
fn ThemeOrbit(
    radius: u32,
    duration: u32,
    reverse: bool,
    nodes: Vec<ThemeNode>,
    on_node_click: EventHandler<usize>,
) -> Element {
    let mut orbit_motion = use_motion(Transform::identity());
    //console::log_1(&format!("Initialized orbit_motion: {:?}", orbit_motion.get_value()).into());

    let orbit_paused = true;

    use_effect(move || {
        if !orbit_paused {
            let rotation_direction = if reverse { -360.0 } else { 360.0 };
            //	console::log_1(&format!("rotation_direction: {}, duration: {}", rotation_direction, duration).into());
            let target_transform = Transform::new(0.0, 0.0, 1.0, rotation_direction);
            //	console::log_1(&format!("Target transform: {:?}", target_transform).into());
            orbit_motion.animate_to(
                target_transform,
                AnimationConfig {
                    mode: AnimationMode::Tween(Tween {
                        duration: Duration::from_secs(duration as u64),
                        easing: easer::functions::Linear::ease_in_out,
                    }),
                    loop_mode: Some(LoopMode::Infinite),
                    ..Default::default()
                },
            );
        }
    });

    let rotation = orbit_motion.get_value().rotation;
    //    console::log_1(&format!("Orbit motion rotation: {}", rotation).into());
    let radius2 = radius / 2;
    let width = radius;
    let height = radius;
    let style = format!(
	"width: {width}px; height: {height}px; margin: -{radius2}px 0 0 -{radius2}px; transform: rotate({rotation}deg);"
    );

    if nodes.is_empty() {
        console::log_1(&"No nodes provided to ThemeOrbit".into());
        return rsx! { div { class: "orbit", style } };
    }
    rsx! {
        div {
            class: "orbit",
            style,
            // Place each node along the orbit
            for (index, node) in nodes.iter().enumerate() {
                ThemeNodeComponent {
                    key: "{index}",
                    node: node.clone(),
                    position_angle: (index as f64 * 360.0) / nodes.len() as f64,
                    radius: radius as f64 / 2.0,
                    on_click: move |_| on_node_click.call(index)
                }
            }
        }
    }
}

// An individual theme node component
#[component]
fn ThemeNodeComponent(
    node: ThemeNode,
    position_angle: f64,
    radius: f64,
    on_click: EventHandler<()>,
) -> Element {
    let mut is_sparking = use_signal(|| false);
    let mut scale = use_motion(1.0f32);

    // Calculate position
    let x = radius * position_angle.to_radians().cos();
    let y = radius * position_angle.to_radians().sin();

    rsx! {
        div {
            class: "theme-node",
            style: "
                left: {x + radius}px; 
                top: {y + radius}px; 
                background-color: {node.color};
                transform: scale({scale.get_value()});
            ",
            onclick: move |_| {
                trigger_node_spark(&mut is_sparking, &mut scale);
                on_click.call(());
            },
            onmouseenter: move |_| {
                scale.animate_to(1.2, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
            },
            onmouseleave: move |_| {
                scale.animate_to(1.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
            },
            "{node.emoji}"

            if is_sparking.read().clone() {
                NodeSpark {
                    emoji: node.emoji.clone(),
                    on_complete: move |_| is_sparking.set(false)
                }
            }
        }
    }
}

// A dazzling spark effect for nodes
#[component]
fn NodeSpark(emoji: String, on_complete: EventHandler<()>) -> Element {
    let mut sparks = use_signal(|| Vec::<SparkParticle>::new());

    use_effect(move || {
        // Create spark particles
        let mut particles = Vec::new();
        for i in 0..10 {
            particles.push(SparkParticle {
                id: i,
                x: 0.0,
                y: 0.0,
                velocity_x: (rand::thread_rng().gen::<f64>() - 0.5) * 200.0,
                velocity_y: (rand::thread_rng().gen::<f64>() - 0.5) * 200.0,
                scale: 1.0 + rand::thread_rng().gen::<f64>() * 2.0,
            });
        }
        sparks.set(particles);

        // Clean up after animation
        spawn(async move {
            TimeoutFuture::new(1_000).await;
            on_complete.call(());
        });
    });

    rsx! {
        div { class: "spark-container",
            for particle in sparks.read().iter() {
                SparkParticleComponent {
                    key: "{particle.id}",
                    emoji: emoji.clone(),
                    particle: particle.clone()
                }
            }
        }
    }
}

// An individual spark particle
#[component]
fn SparkParticleComponent(emoji: String, particle: SparkParticle) -> Element {
    let mut motion = use_motion(Transform::identity());
    let mut opacity = use_motion(1.0f32);

    use_effect(move || {
        motion.animate_to(
            Transform::new(
                particle.velocity_x as f32,
                particle.velocity_y as f32,
                (particle.scale * 3.0) as f32,
                0.0,
            ),
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Linear::ease_in_out,
                ..Default::default()
            })),
        );
        opacity.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Linear::ease_in_out,
                ..Default::default()
            })),
        );
    });

    let transform = motion.get_value();
    let opac = opacity.get_value();
    let style = format!(
        "position: absolute; font-size: 2em; pointer-events: none; transform: translate({}px, {}px) scale({}) rotate({}deg); opacity: {opac};",
        transform.x,
        transform.y,
        transform.scale,
        transform.rotation
    );

    rsx! {
        div {
            class: "spark-particle",
            style,
            "{emoji}"
        }
    }
}

// The central boost core component
#[component]
fn BoostCore(
    boost_active: bool,
    current_emoji_sequence: usize,
    on_boost: EventHandler<()>,
) -> Element {
    let mut scale = use_motion(1.0f32);
    let emoji_sequences = get_emoji_sequences();
    let mut rotation = use_motion(0.0f32);
    let easing = easer::functions::Elastic::ease_out;

    use_effect(move || {
        if boost_active {
            let sequence = AnimationSequence::new()
                .then(
                    1.3,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing,
                    })),
                )
                .then(
                    1.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing,
                    })),
                );
            scale.animate_sequence(sequence);
        }
    });
    // Animate boost effect when active - matches HTML hyperPump keyframes
    use_effect(move || {
        if boost_active {
            console::log_1(&"🚀 BOOST ANIMATION TRIGGERED! 🚀".into());
            let sequence = AnimationSequence::new()
                .then(
                    1.1,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    1.2,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    1.1,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    1.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                );
            scale.animate_sequence(sequence);

            // Add rotation animation
            let rotation_sequence = AnimationSequence::new()
                .then(
                    2.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    0.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    -2.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                )
                .then(
                    0.0,
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(500),
                        easing: easer::functions::Elastic::ease_out,
                    })),
                );
            rotation.animate_sequence(rotation_sequence);
        }
    });

    rsx! {
        div {
            class: "boost-core",
            style: "transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg)",
            onclick: move |_| on_boost.call(()),

            h1 { class: "title", "SOLFUNTHEME" }
            h2 { class: "subtitle", "Zero Ontology System" }

            div {
                class: "emoji-engine",
                "{emoji_sequences[current_emoji_sequence % emoji_sequences.len()]}"
            }

            div { class: "core-description",
                div { class: "glow-text", "Super-Theme Boost Protocol" }
                div { class: "core-tags", "Self-Reflective • Iterative • Fun" }
            }
        }
    }
}

// A panel showcasing system features
#[component]
fn FeaturesPanel() -> Element {
    let features = get_system_features();

    rsx! {
        div { class: "features-panel",
            h3 { "Key Features" }
            for feature in features {
                FeatureItem {
                    emoji: feature.emoji.clone(),
                    text: feature.text.clone()
                }
            }
        }
    }
}

// An individual feature item
#[component]
fn FeatureItem(emoji: String, text: String) -> Element {
    let mut rotation = use_motion(0.0f32);

    // Gentle wiggle animation - matches HTML wiggle with ping-pong effect
    use_effect(move || {
        rotation.animate_to(
            5.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1500),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite), // back-and-forth animation
                                            //            .with_loop(LoopMode::Alternate)  // back-and-forth animation
                                            //loop_mode: Some(LoopMode::Infinite),
        );
    });

    rsx! {
        div { class: "feature",
            span {
                class: "feature-emoji",
                style: "transform: rotate({rotation.get_value()}deg)",
                "{emoji}"
            }
            span { "{text}" }
        }
    }
}

// A panel displaying community agreement metrics
#[component]
fn AgreementPanel(
    fun_score: f64,
    theme_speed: String,
    agreement: f64,
    excitement_level: String,
) -> Element {
    rsx! {
        div { class: "agreement-panel",
            h3 { "Community Agreement" }

            MetricDisplay {
                label: "Fun Score:".to_string(),
                value: format!("{:.1}K", fun_score),
                color: "#00ff00".to_string()
            }
            MetricDisplay {
                label: "Theme Speed:".to_string(),
                value: theme_speed,
                color: "#ffff00".to_string()
            }
            MetricDisplay {
                label: "Agreement:".to_string(),
                value: format!("{:.1}% BOOST", agreement),
                color: "#00ffff".to_string()
            }
            MetricDisplay {
                label: "Excitement Level:".to_string(),
                value: excitement_level,
                color: "#ff00ff".to_string()
            }
        }
    }
}

// A panel explaining the workflow
#[component]
fn WorkflowPanel() -> Element {
    let workflow_steps = get_workflow_steps();

    rsx! {
        div { class: "workflow-panel",
            h3 { "How It Works" }
            for step in workflow_steps {
                WorkflowStep {
                    number: step.number.clone(),
                    title: step.title.clone(),
                    description: step.description.clone(),
                    color: step.color.clone()
                }
            }
        }
    }
}

// An individual workflow step
#[component]
fn WorkflowStep(number: String, title: String, description: String, color: String) -> Element {
    rsx! {
        div { class: "workflow-step",
            span { style: "color: {color};", "{number}" }
            " {title}"
            br {}
            small { "{description}" }
        }
    }
}

// A panel for live boost metrics
#[component]
fn BoostMetricsPanel() -> Element {
    let mut metrics = use_signal(|| get_initial_metrics());

    // Update metrics periodically
    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(2_000).await;
                update_boost_metrics(&mut metrics);
            }
        });
    });

    rsx! {
        div { class: "boost-metrics",
            h3 { "Live Metrics" }
            for metric in metrics.read().iter() {
                MetricDisplay {
                    label: metric.label.clone(),
                    value: metric.value.clone(),
                    color: "#00ffff".to_string()
                }
            }
        }
    }
}

// A reusable metric display component
#[component]
fn MetricDisplay(label: String, value: String, color: String) -> Element {
    rsx! {
        div { class: "metric",
            span { "{label}" }
            " "
            span {
                class: "metric-value",
                style: "color: {color};",
                "{value}"
            }
        }
    }
}

// Global event handlers for keyboard input
#[component]
fn GlobalEventHandlers(on_space_pressed: EventHandler<()>) -> Element {
    use_effect(move || {
        let window = window().unwrap();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.code() == "Space" {
                    event.prevent_default();
                    on_space_pressed.call(());
                }
            }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    rsx! { div { hidden: true } }
}

// Data structures
#[derive(Clone, Debug, PartialEq)]
struct ThemeNode {
    emoji: String,
    color: String,
}

#[derive(Clone, Debug, PartialEq)]
struct ParticleState {
    id: u64,
    x: f64,
    y: f64,
    color: String,
    duration: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct SparkParticle {
    id: usize,
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    scale: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct SystemFeature {
    emoji: String,
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
struct WorkflowStepData {
    number: String,
    title: String,
    description: String,
    color: String,
}

#[derive(Clone, Debug, PartialEq)]
struct MetricData {
    label: String,
    value: String,
}

// Utility functions
fn get_orbit_nodes(orbit_index: usize) -> Vec<ThemeNode> {
    match orbit_index {
        0 => vec![
            ThemeNode {
                emoji: "🚀".to_string(),
                color: "rgba(255, 0, 0, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "📜".to_string(),
                color: "rgba(255, 255, 0, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "🔍".to_string(),
                color: "rgba(0, 255, 255, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "💬".to_string(),
                color: "rgba(255, 0, 255, 0.8)".to_string(),
            },
        ],
        1 => vec![
            ThemeNode {
                emoji: "🔀".to_string(),
                color: "rgba(0, 255, 0, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "💡".to_string(),
                color: "rgba(255, 128, 0, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "💭".to_string(),
                color: "rgba(128, 255, 0, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "🔑".to_string(),
                color: "rgba(255, 0, 128, 0.8)".to_string(),
            },
        ],
        2 => vec![
            ThemeNode {
                emoji: "🤖".to_string(),
                color: "rgba(128, 0, 255, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "🌐".to_string(),
                color: "rgba(0, 128, 255, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "📊".to_string(),
                color: "rgba(255, 255, 128, 0.8)".to_string(),
            },
            ThemeNode {
                emoji: "🔗".to_string(),
                color: "rgba(128, 255, 255, 0.8)".to_string(),
            },
        ],
        _ => vec![],
    }
}

fn get_emoji_sequences() -> Vec<String> {
    vec![
        "🚀📜🔍💬🧠".to_string(),
        "🔀💡💭🔑".to_string(),
        "🤖🌐📊🔗".to_string(),
        "🧩🔗🌱".to_string(),
        "💎🚁💰✨".to_string(),
        "🌙📈💫⚡".to_string(),
        "🎯🎪🎨🎭".to_string(),
        "🔮✨🌟💫".to_string(),
    ]
}

fn get_system_features() -> Vec<SystemFeature> {
    vec![
        SystemFeature {
            emoji: "🚀".to_string(),
            text: "Self-Reflective Theme Engine".to_string(),
        },
        SystemFeature {
            emoji: "🔀".to_string(),
            text: "Community Agreement System".to_string(),
        },
        SystemFeature {
            emoji: "📈".to_string(),
            text: "Super Boost Mechanism".to_string(),
        },
        SystemFeature {
            emoji: "📜".to_string(),
            text: "Theme Compression".to_string(),
        },
        SystemFeature {
            emoji: "🔗".to_string(),
            text: "Stable Theme-State".to_string(),
        },
        SystemFeature {
            emoji: "🌱".to_string(),
            text: "Theme Growth & Sharing".to_string(),
        },
    ]
}

fn get_workflow_steps() -> Vec<WorkflowStepData> {
    vec![
        WorkflowStepData {
            number: "1️⃣".to_string(),
            title: "ZOS Interaction".to_string(),
            description: "Engage with fun theme creation".to_string(),
            color: "#00ff00".to_string(),
        },
        WorkflowStepData {
            number: "2️⃣".to_string(),
            title: "Community Agreement".to_string(),
            description: "Shape themes together".to_string(),
            color: "#ff00ff".to_string(),
        },
        WorkflowStepData {
            number: "3️⃣".to_string(),
            title: "Self-Enhancement".to_string(),
            description: "Grows with fun & engagement".to_string(),
            color: "#00ffff".to_string(),
        },
        WorkflowStepData {
            number: "4️⃣".to_string(),
            title: "Fun Propagation".to_string(),
            description: "Spread exciting narratives".to_string(),
            color: "#ffff00".to_string(),
        },
    ]
}

fn get_initial_metrics() -> Vec<MetricData> {
    vec![
        MetricData {
            label: "Theme Strength:".to_string(),
            value: "SUPER RARE".to_string(),
        },
        MetricData {
            label: "Boost Factor:".to_string(),
            value: "∞x".to_string(),
        },
        MetricData {
            label: "Fun Coefficient:".to_string(),
            value: "1.847".to_string(),
        },
        MetricData {
            label: "Theme Level:".to_string(),
            value: "ITERATIVE".to_string(),
        },
        MetricData {
            label: "Status:".to_string(),
            value: "BOOSTING".to_string(),
        },
    ]
}

// Event handler functions
fn update_metrics(fun_score: &mut Signal<f64>, agreement: &mut Signal<f64>) {
    let mut rng = rand::thread_rng();
    let current_score = fun_score.read().clone();
    fun_score.set(current_score + rng.gen_range(-5.0..10.0));

    let current_agreement = agreement.read().clone();
    agreement.set((current_agreement + rng.gen_range(-0.5..0.5)).clamp(95.0, 100.0));
}

fn rotate_emoji_sequence(current_sequence: &mut Signal<usize>) {
    let sequences = get_emoji_sequences();
    let current = current_sequence.read().clone();
    current_sequence.set((current + 1) % sequences.len());
}

fn spawn_particle(particles: &mut Signal<Vec<ParticleState>>) {
    let mut rng = rand::thread_rng();
    let colors = ["#00ff00", "#ff00ff", "#00ffff", "#ffff00", "#ff0080"];

    let new_particle = ParticleState {
        id: rng.gen::<u64>(),
        x: rng.gen_range(0.0..100.0),
        y: 100.0,
        color: colors[rng.gen_range(0..colors.len())].to_string(),
        duration: rng.gen_range(4000..8000),
    };

    let mut current_particles = particles.read().clone();
    current_particles.push(new_particle);

    // Keep only recent particles
    if current_particles.len() > 20 {
        current_particles.remove(0);
    }

    particles.set(current_particles);
}

fn handle_node_click(selected_node: &mut Signal<Option<usize>>, node_id: usize) {
    selected_node.set(Some(node_id));
    console::log_1(&format!("Node {} clicked!", node_id).into());
}

fn trigger_node_spark(is_sparking: &mut Signal<bool>, scale: &mut impl AnimationManager<f32>) {
    is_sparking.set(true);

    // Trigger node shake effect
    scale.animate_to(
        1.3,
        AnimationConfig::new(AnimationMode::Spring(Spring::default())),
    );
}

fn trigger_super_boost(boost_active: &mut Signal<bool>) {
    boost_active.set(true);

    // Reset boost state after animation
    let mut boost_active_clone = boost_active.clone();
    spawn(async move {
        TimeoutFuture::new(1_000).await;
        boost_active_clone.set(false);
    });
}

fn trigger_mega_boost_with_particles(
    boost_active: &mut Signal<bool>,
    particles: &mut Signal<Vec<ParticleState>>,
) {
    boost_active.set(true);
    console::log_1(&"🚀 MEGA BOOST ACTIVATED! 🚀".into());

    // Spawn particles for mega boost effect
    for _ in 0..50 {
        spawn_particle(particles);
    }

    // Create screen flash effect
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.style().set_property(
            "background",
            "linear-gradient(45deg, #ff0000, #ff00ff, #00ffff, #00ff00)",
        );
        let _ = body.style().set_property("background-size", "800% 800%");

        // Reset after mega boost
        let body_clone = body.clone();
        spawn(async move {
            TimeoutFuture::new(2_000).await;
            let _ = body_clone.style().set_property(
                "background",
                "linear-gradient(45deg, #000000, #1a0033, #330066, #6600ff)",
            );
            let _ = body_clone
                .style()
                .set_property("background-size", "400% 400%");
        });
    }

    // Reset boost state
    let mut boost_active_clone = boost_active.clone();
    spawn(async move {
        TimeoutFuture::new(2_000).await;
        boost_active_clone.set(false);
    });
}

fn trigger_mega_boost(boost_active: &mut Signal<bool>) {
    boost_active.set(true);
    console::log_1(&"🚀 MEGA BOOST ACTIVATED! 🚀".into());

    // Create screen flash effect
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        let _ = body.style().set_property(
            "background",
            "linear-gradient(45deg, #ff0000, #ff00ff, #00ffff, #00ff00)",
        );
        let _ = body.style().set_property("background-size", "800% 800%");

        // Reset after mega boost
        let body_clone = body.clone();
        spawn(async move {
            TimeoutFuture::new(2_000).await;
            let _ = body_clone.style().set_property(
                "background",
                "linear-gradient(45deg, #000000, #1a0033, #330066, #6600ff)",
            );
            let _ = body_clone
                .style()
                .set_property("background-size", "400% 400%");
        });
    }

    // Reset boost state
    let mut boost_active_clone = boost_active.clone();
    spawn(async move {
        TimeoutFuture::new(2_000).await;
        boost_active_clone.set(false);
    });
}

fn update_boost_metrics(metrics: &mut Signal<Vec<MetricData>>) {
    let mut rng = rand::thread_rng();
    let mut current_metrics = metrics.read().clone();

    // Update Fun coefficient
    for metric in &mut current_metrics {
        match metric.label.as_str() {
            "Fun Coefficient:" => {
                let new_value = 1.0 + rng.gen_range(0.0..0.5);
                metric.value = format!("{:.3}", new_value);
            }
            "Theme Strength:" => {
                let strengths = ["SUPER RARE", "ULTRA RARE", "LEGENDARY", "MYTHIC"];
                metric.value = strengths[rng.gen_range(0..strengths.len())].to_string();
            }
            "Boost Factor:" => {
                let factors = ["∞x", "∞∞x", "∞∞∞x", "MAX BOOST"];
                metric.value = factors[rng.gen_range(0..factors.len())].to_string();
            }
            "Theme Level:" => {
                let levels = ["ITERATIVE", "ADAPTIVE", "EVOLVING", "TRANSCENDENT"];
                metric.value = levels[rng.gen_range(0..levels.len())].to_string();
            }
            "Status:" => {
                let statuses = ["BOOSTING", "POWERING UP", "MAXIMUM FUN", "OVERDRIVE"];
                metric.value = statuses[rng.gen_range(0..statuses.len())].to_string();
            }
            _ => {}
        }
    }

    metrics.set(current_metrics);
}
