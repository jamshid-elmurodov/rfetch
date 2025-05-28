use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub(crate) struct GPU {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_brand: bool,
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            show: true,
            show_brand: true,
        }
    }
}