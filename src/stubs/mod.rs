//! Stubs for dioxus-motion (not available for dioxus 0.7)
pub mod motion {
    use std::time::Duration;

    pub mod prelude {
        pub use super::*;
    }

    pub mod animations {
        pub mod utils {
            #[derive(Clone, Copy)]
            pub enum LoopMode {
                Infinite,
                Alternate,
                Times(u32),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Transform {
        pub x: f32,
        pub y: f32,
        pub scale: f32,
        pub rotation: f32,
    }

    impl Transform {
        pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Self {
            Self { x, y, scale, rotation }
        }
        pub fn identity() -> Self {
            Self { x: 0.0, y: 0.0, scale: 1.0, rotation: 0.0 }
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct Spring;

    impl Spring {
        pub fn default() -> Self { Self }
    }

    #[derive(Clone, Copy)]
    pub struct Tween {
        pub duration: Duration,
        pub easing: fn(f32, f32, f32, f32) -> f32,
    }

    impl Default for Tween {
        fn default() -> Self {
            Self {
                duration: Duration::from_millis(300),
                easing: |t, b, c, d| t / d * c + b,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum AnimationMode {
        Tween(Tween),
        Spring(Spring),
    }

    #[derive(Clone, Copy, Default)]
    pub struct AnimationConfig {
        pub loop_mode: Option<()>,
    }

    impl AnimationConfig {
        pub fn new(_mode: AnimationMode) -> Self { Self { loop_mode: None } }
        pub fn with_loop(self, _mode: animations::utils::LoopMode) -> Self { self }
    }

    #[derive(Clone)]
    pub struct AnimationSequence;

    impl AnimationSequence {
        pub fn new() -> Self { Self }
        pub fn then(self, _target: impl Clone, _config: AnimationConfig) -> Self { self }
    }

    pub trait AnimationManager<T: Clone> {
        fn animate_to(&mut self, target: T, config: AnimationConfig);
        fn animate_sequence(&mut self, seq: AnimationSequence);
        fn get_value(&self) -> T;
    }

    pub struct MotionHandleF32 { pub value: f32 }
    pub struct MotionHandleTransform { pub value: Transform }

    impl MotionHandleF32 {
        pub fn animate_to(&mut self, _target: f32, _config: AnimationConfig) {}
        pub fn animate_sequence(&mut self, _seq: AnimationSequence) {}
        pub fn get_value(&self) -> f32 { self.value }
    }

    impl MotionHandleTransform {
        pub fn animate_to(&mut self, _target: Transform, _config: AnimationConfig) {}
        pub fn animate_sequence(&mut self, _seq: AnimationSequence) {}
        pub fn get_value(&self) -> Transform { self.value }
    }

    pub fn use_motion_f32(_initial: f32) -> MotionHandleF32 {
        MotionHandleF32 { value: 0.0 }
    }

    pub fn use_motion_transform(_initial: Transform) -> MotionHandleTransform {
        MotionHandleTransform { value: Transform::identity() }
    }

    // Generic use_motion — callers use the typed versions above
    // but we keep this for compatibility
    pub fn use_motion<T: Clone + Default>(_initial: T) -> MotionHandleF32 {
        MotionHandleF32 { value: 0.0 }
    }
}
