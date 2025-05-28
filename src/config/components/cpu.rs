use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub(crate) struct CPU {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_brand: bool,
    #[serde(default = "default_true")]
    show_cores: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            show: true,
            show_brand: true,
            show_cores: true,
        }
    }
}