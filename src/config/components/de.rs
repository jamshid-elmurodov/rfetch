use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DE {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_version: bool,
}

impl Default for DE {
    fn default() -> Self {
        Self {
            show: true,
            show_version: true,
        }
    }
}