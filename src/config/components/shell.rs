use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Shell {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_path: bool,
    #[serde(default = "default_true")]
    show_version: bool,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            show: true,
            show_path: true,
            show_version: true,
        }
    }
}