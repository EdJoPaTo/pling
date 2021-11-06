#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/desktop.md")]
pub struct Desktop {
    pub summary: Option<String>,
}

impl From<Desktop> for crate::Notifier {
    fn from(desktop: Desktop) -> Self {
        Self::Desktop(desktop)
    }
}

impl Desktop {
    #[must_use]
    /// Loads the desktop notification config from environment variables.
    /// The following variables are used:
    /// - `PLING_DESKTOP_ENABLED`
    /// - `PLING_DESKTOP_SUMMARY`
    pub fn from_env() -> Option<Self> {
        let enabled = std::env::var_os("PLING_DESKTOP_ENABLED").is_some();
        let summary = std::env::var("PLING_DESKTOP_SUMMARY").ok();

        if enabled || summary.is_some() {
            Some(Self { summary })
        } else {
            None
        }
    }

    /// Send a Desktop notification.
    ///
    /// # Errors
    ///
    /// This method errors when the notification could not be displayed.
    pub fn send_sync(&self, text: &str) -> Result<(), notify_rust::error::Error> {
        let mut n = notify_rust::Notification::new();

        if let Some(summary) = &self.summary {
            n.summary(summary);
        }

        n.body(text).show()?;
        Ok(())
    }
}
