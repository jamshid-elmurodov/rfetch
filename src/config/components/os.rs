use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OS {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_os_arch: bool,
}

impl Default for OS {
    fn default() -> Self {
        Self {
            show: true,
            show_os_arch: true,
        }
    }
}