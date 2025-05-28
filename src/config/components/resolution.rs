use serde::Deserialize;
use crate::config::util::default_true;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub(crate) struct Resolution {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_refresh_rate: bool,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            show: true,
            show_refresh_rate: true,
        }
    }
}