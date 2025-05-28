use crate::config::unit::Unit;
use crate::config::util::default_true;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Memory {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_percent: bool,
    unit: Unit,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            show: true,
            show_percent: true,
            unit: Unit::GiB,
        }
    }
}
