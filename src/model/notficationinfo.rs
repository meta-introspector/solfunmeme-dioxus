
#[derive(Debug, Clone)]
pub struct NotificationInfo {
    key: u32,
    secs: u32,
    message: String,
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

    pub fn key(&self) -> u32 {
        self.key
    }

    pub fn secs(&self) -> u32 {
        self.secs
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
