use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Disk {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_percent: bool,
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            show: true,
            show_percent: true,
        }
    }
}