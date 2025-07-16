use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Signal state wrapper for better type safety
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalState<T: 'static> {
    pub value: T,
    pub is_loading: bool,
    pub error: Option<String>,
    pub last_updated: std::time::SystemTime,
}

impl<T: Clone + 'static> SignalState<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            is_loading: false,
            error: None,
            last_updated: std::time::SystemTime::now(),
        }
    }

    pub fn loading(mut self) -> Self {
        self.is_loading = true;
        self
    }

    pub fn error(mut self, error: String) -> Self {
        self.is_loading = false;
        self.error = Some(error);
        self
    }

    pub fn success(mut self, value: T) -> Self {
        self.is_loading = false;
        self.error = None;
        self.value = value;
        self.last_updated = std::time::SystemTime::now();
        self
    }
}

/// Simple signal manager for managing state with loading and error handling
#[derive(Clone)]
pub struct SignalManager<T: 'static> {
    state: Signal<SignalState<T>>,
}

impl<T: Clone + 'static> SignalManager<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            state: Signal::new(SignalState::new(initial_value)),
        }
    }

    pub fn set_loading(&mut self) {
        let current = self.state.read().clone();
        self.state.set(current.loading());
    }

    pub fn set_error(&mut self, error: String) {
        let current = self.state.read().clone();
        self.state.set(current.error(error));
    }

    pub fn set_success(&mut self, value: T) {
        let current = self.state.read().clone();
        self.state.set(current.success(value));
    }

    pub fn update<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut T)
    {
        let mut current = self.state.read().clone();
        f(&mut current.value);
        current.last_updated = std::time::SystemTime::now();
        self.state.set(current);
    }

    pub fn get(&self) -> T {
        self.state.read().value.clone()
    }

    pub fn get_state(&self) -> SignalState<T> {
        self.state.read().clone()
    }

    pub fn subscribe(&self) -> Signal<SignalState<T>> {
        self.state.clone()
    }

    pub fn is_loading(&self) -> bool {
        self.state.read().is_loading
    }

    pub fn has_error(&self) -> bool {
        self.state.read().error.is_some()
    }

    pub fn get_error(&self) -> Option<String> {
        self.state.read().error.clone()
    }
}

/// Pattern for creating signal managers in components
pub fn use_signal_manager<T: Clone + 'static>(initial_value: T) -> SignalManager<T> {
    SignalManager::new(initial_value)
}

/// Simple signal manager for basic state management
pub struct SimpleSignalManager<T: 'static> {
    signal: Signal<T>,
}

impl<T: Clone + 'static> SimpleSignalManager<T> {
    pub fn new(initial: T) -> Self {
        Self {
            signal: Signal::new(initial),
        }
    }

    pub fn get(&self) -> T {
        self.signal.read().clone()
    }

    pub fn set(&mut self, value: T) {
        self.signal.set(value);
    }

    pub fn update<F>(&mut self, f: F) where F: FnOnce(&mut T) {
        let mut value = self.signal.read().clone();
        f(&mut value);
        self.signal.set(value);
    }

    pub fn subscribe(&self) -> Signal<T> {
        self.signal.clone()
    }
}

/// Async signal manager for handling async operations
pub struct AsyncSignalManager<T: 'static> {
    signal: Signal<SignalState<T>>,
}

impl<T: Clone + 'static> AsyncSignalManager<T> {
    pub fn new(initial: T) -> Self {
        Self {
            signal: Signal::new(SignalState::new(initial)),
        }
    }

    pub fn get(&self) -> SignalState<T> {
        self.signal.read().clone()
    }

    pub fn set_loading(&mut self) {
        let mut state = self.signal.read().clone();
        state.is_loading = true;
        self.signal.set(state);
    }

    pub fn set_error(&mut self, error: String) {
        let mut state = self.signal.read().clone();
        state.is_loading = false;
        state.error = Some(error);
        self.signal.set(state);
    }

    pub fn set_success(&mut self, value: T) {
        let mut state = self.signal.read().clone();
        state.value = value;
        state.is_loading = false;
        state.error = None;
        state.last_updated = std::time::SystemTime::now();
        self.signal.set(state);
    }

    pub fn subscribe(&self) -> Signal<SignalState<T>> {
        self.signal.clone()
    }
}

/// Helper macro for creating typed signal managers
#[macro_export]
macro_rules! create_signal_manager {
    ($name:ident, $type:ty, $initial:expr) => {
        pub struct $name;
        
        impl $name {
            pub fn get() -> $type {
                // This would need to be implemented with a proper global signal
                $initial
            }
            
            pub fn set(value: $type) {
                // This would need to be implemented with a proper global signal
            }
            
            pub fn update<F>(f: F) where F: FnOnce(&mut $type) {
                // This would need to be implemented with a proper global signal
            }
            
            pub fn subscribe() -> Signal<$type> {
                // This would need to be implemented with a proper global signal
                Signal::new($initial)
            }
        }
    };
} 