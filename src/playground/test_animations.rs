use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use std::time::Duration;
use easer::functions::Easing;

// Simple test component to verify animations work
#[component]
pub fn TestAnimations() -> Element {
    let mut scale = use_motion(1.0f32);
    let mut rotation = use_motion(0.0f32);

    // Test basic animation
    use_effect(move || {
        scale.animate_to(
            2.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Elastic::ease_out,
                ..Default::default()
            }))
            .with_loop(LoopMode::Alternate)
        );
        
        rotation.animate_to(
            360.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(2),
                easing: easer::functions::Linear::ease_in_out,
                ..Default::default()
            }))
            .with_loop(LoopMode::Infinite)
        );
    });

    rsx! {
        div {
            style: "
                width: 100vw;
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: linear-gradient(45deg, #000000, #1a0033, #330066, #6600ff);
                color: white;
                font-family: 'Courier New', monospace;
            ",
            
            div {
                style: "
                    padding: 40px;
                    background: rgba(102, 0, 255, 0.1);
                    border: 3px solid #00ff00;
                    border-radius: 20px;
                    text-align: center;
                    transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg);
                    transition: all 0.3s ease;
                ",
                
                h1 { "Animation Test" }
                p { "Scale: {scale.get_value():.2}" }
                p { "Rotation: {rotation.get_value():.1}°" }
                p { "✅ Animations working!" }
            }
        }
    }
}

// Test the main app
#[component]
pub fn TestSolFunNiceApp() -> Element {
    rsx! {
        div {
            style: "width: 100vw; height: 100vh;",
            SolFunNiceApp {}
        }
    }
} 