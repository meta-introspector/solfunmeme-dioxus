use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationInfo {
    pub key: u32,
    pub secs: u32,
    pub message: String,
    pub notification_type: NotificationType,
    pub created_at: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationInfo {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            key: fastrand::u32(..),
            secs: 5,
            message: message.into(),
            notification_type: NotificationType::Info,
            created_at: std::time::SystemTime::now(),
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            key: fastrand::u32(..),
            secs: 5,
            message: message.into(),
            notification_type: NotificationType::Success,
            created_at: std::time::SystemTime::now(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            key: fastrand::u32(..),
            secs: 10,
            message: message.into(),
            notification_type: NotificationType::Warning,
            created_at: std::time::SystemTime::now(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            key: fastrand::u32(..),
            secs: 15,
            message: message.into(),
            notification_type: NotificationType::Error,
            created_at: std::time::SystemTime::now(),
        }
    }

    pub fn set_duration(mut self, secs: u32) -> Self {
        self.secs = secs;
        self
    }
}

/// Notification manager for managing notifications
pub struct NotificationManager {
    notifications: AsyncSignalManager<VecDeque<NotificationInfo>>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: AsyncSignalManager::new(VecDeque::new()),
        }
    }

    pub fn add_notification(&mut self, notification: NotificationInfo) {
        let mut notifications = self.notifications.get().value;
        notifications.push_back(notification);
        self.notifications.set_success(notifications);
    }

    pub fn remove_notification(&mut self, key: u32) {
        let mut notifications = self.notifications.get().value;
        notifications.retain(|n| n.key != key);
        self.notifications.set_success(notifications);
    }

    pub fn clear_all(&mut self) {
        self.notifications.set_success(VecDeque::new());
    }

    pub fn get_notifications(&self) -> VecDeque<NotificationInfo> {
        self.notifications.get().value
    }

    pub fn subscribe(&self) -> Signal<SignalState<VecDeque<NotificationInfo>>> {
        self.notifications.subscribe()
    }

    // Convenience methods
    pub fn info(&mut self, message: impl Into<String>) {
        self.add_notification(NotificationInfo::new(message));
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.add_notification(NotificationInfo::success(message));
    }

    pub fn warning(&mut self, message: impl Into<String>) {
        self.add_notification(NotificationInfo::warning(message));
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.add_notification(NotificationInfo::error(message));
    }
}

// Convenience functions for creating notification managers
pub fn create_notification_manager() -> NotificationManager {
    NotificationManager::new()
}

// Example usage functions (these would need to be called with a manager instance)
pub fn show_info(manager: &mut NotificationManager, message: impl Into<String>) {
    manager.info(message);
}

pub fn show_success(manager: &mut NotificationManager, message: impl Into<String>) {
    manager.success(message);
}

pub fn show_warning(manager: &mut NotificationManager, message: impl Into<String>) {
    manager.warning(message);
}

pub fn show_error(manager: &mut NotificationManager, message: impl Into<String>) {
    manager.error(message);
} 