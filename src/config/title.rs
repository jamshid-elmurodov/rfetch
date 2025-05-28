use crate::config::{
    color::Color,
    constants::{
        DEFAULT_COLOR_OF_TITLE, DEFAULT_COLOR_OF_TITLES_SEPARATOR, DEFAULT_COLOR_OF_UNDERLINE,
    },
    util::default_true,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Title {
    #[serde(default = "default_true")]
    show: bool,
    color: Color,
    #[serde(default = "default_true")]
    bold: bool,
    #[serde(default = "default_true")]
    show_underline: bool,
    underline_color: Color,
    separator_color: Color,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            show: true,
            color: DEFAULT_COLOR_OF_TITLE,
            bold: true,
            show_underline: true,
            underline_color: DEFAULT_COLOR_OF_UNDERLINE,
            separator_color: DEFAULT_COLOR_OF_TITLES_SEPARATOR,
        }
    }
}
