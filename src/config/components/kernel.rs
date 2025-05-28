use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Kernel {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    shorthand: bool,
}

impl Default for Kernel {
    fn default() -> Self {
        Self {
            show: true,
            shorthand: false,
        }
    }
}