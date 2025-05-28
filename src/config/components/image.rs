use serde::Deserialize;
use crate::config::color::Color;
use crate::config::constants::DEFAULT_COLOR_OF_IMAGE;
use crate::config::util::default_source;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Image {
    color: Color,
    #[serde(default = "default_source")]
    source: String,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR_OF_IMAGE,
            source: default_source(),
        }
    }
}