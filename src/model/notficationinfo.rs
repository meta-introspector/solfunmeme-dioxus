#[derive(Debug, Clone)]
pub struct NotificationInfo {
    #[allow(dead_code)]
    pub key: u32,
    pub secs: u32,
    #[allow(dead_code)]
    pub message: String,
}

impl NotificationInfo {
    pub fn new(message: impl core::fmt::Display) -> Self {
        let key = fastrand::u32(..);

        Self {
            key,
            secs: 2,
            message: message.to_string(),
        }
    }

    /// Sets default seconds to 15
    pub fn error(message: impl core::fmt::Display) -> Self {
        Self::new(message).set_secs(15)
    }

    pub fn set_secs(mut self, secs: u32) -> Self {
        self.secs = secs;

        self
    }
    #[allow(dead_code)]
    pub fn key(&self) -> u32 {
        self.key
    }
    #[allow(dead_code)]
    pub fn secs(&self) -> u32 {
        self.secs
    }
    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

//#[cfg(test)]
mod tests {
    

    #[test]
    fn test_new_notification_info_defaults() {
        let notif = NotificationInfo::new("Hello");
        assert_eq!(notif.secs(), 2);
        assert_eq!(notif.message(), "Hello");
    }

    #[test]
    fn test_error_notification_info() {
        let notif = NotificationInfo::error("Error occurred");
        assert_eq!(notif.secs(), 15);
        assert_eq!(notif.message(), "Error occurred");
    }

    #[test]
    fn test_set_secs() {
        let notif = NotificationInfo::new("Test").set_secs(10);
        assert_eq!(notif.secs(), 10);
    }

    #[test]
    fn test_key_is_random() {
        let notif1 = NotificationInfo::new("A");
        let notif2 = NotificationInfo::new("B");
        // It's possible but very unlikely for two random u32s to be equal
        assert_ne!(notif1.key(), notif2.key());
    }

    #[test]
    fn test_message_is_stored_correctly() {
        let msg = "Some message";
        let notif = NotificationInfo::new(msg);
        assert_eq!(notif.message(), msg);
    }

    #[test]
    fn test_clone_trait() {
        let notif = NotificationInfo::new("Clone me");
        let cloned = notif.clone();
        assert_eq!(notif.key(), cloned.key());
        assert_eq!(notif.secs(), cloned.secs());
        assert_eq!(notif.message(), cloned.message());
    }
}
