use dioxus::prelude::*;

// Simplified notification component
#[derive(Debug, Clone)]
pub struct NotificationInfo {
    pub key: u32,
    pub secs: u32,
    pub message: String,
}

impl NotificationInfo {
    pub fn key(&self) -> u32 { self.key }
    pub fn secs(&self) -> u32 { self.secs }
    pub fn message(&self) -> &str { &self.message }
}

// Global notification storage using Dioxus signals
static NOTIFICATIONS: GlobalSignal<Vec<NotificationInfo>> = Signal::global(Vec::new);

#[component]
pub fn Notification() -> Element {
    let notifications = NOTIFICATIONS.read().clone();
    
    rsx! {
        div {
            class: "cursor-pointer fixed z-1000 top-4 right-4 flex flex-col space-y-2 min-w-[300px] shadow-lg",
            for notification_info in notifications {
                div {
                    onclick: move |_| {
                        remove_notification(notification_info.key);
                    },
                    key: "{notification_info.key}",
                    class: "flex border dark:border-none items-center opacity-0 translate-y-4 animate-fade-in w-full max-w-xs p-2 space-x-2 text-gray-600 bg-white divide-x divide-gray-200 rounded-lg shadow-sm dark:text-gray-400 dark:divide-gray-700 dark:bg-gray-800",
                    div { 
                        class: "flex w-[30px]",
                        svg {
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "m10 20h4a2 2 0 0 1 -4 0zm8-4v-6a6 6 0 0 0 -5-5.91v-1.09a1 1 0 0 0 -2 0v1.09a6 6 0 0 0 -5 5.91v6l-2 2h16z",
                                fill: "#0060df",
                            }
                        }
                    }
                    div { 
                        class: "ps-4 text-sm font-normal", 
                        "{notification_info.message}" 
                    }
                }
            }
        }
    }
}

// Helper functions for backward compatibility
pub fn show_notification(message: String, secs: u32) {
    let key = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    
    let notification = NotificationInfo { key, secs, message };
    NOTIFICATIONS.write().push(notification);
}

pub fn remove_notification(key: u32) {
    NOTIFICATIONS.write().retain(|n| n.key != key);
} 