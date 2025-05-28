use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Uptime {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    shorthand: bool,
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            show: true,
            shorthand: false,
        }
    }
}