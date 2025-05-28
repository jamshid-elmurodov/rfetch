use crate::config::color::Color;
use crate::config::constants::{
    DEFAULT_COLOR_OF_INFOS, DEFAULT_COLOR_OF_SUBTITLES, DEFAULT_COLOR_OF_SUBTITLES_SEPARATOR,
};
use crate::config::util::{default_separator, default_true};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Subtitles {
    #[serde(default = "default_true")]
    subtitle_bold: bool,
    subtitle_color: Color,
    separator: Separator,
    info_color: Color,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Separator {
    #[serde(default = "default_separator")]
    char: char,
    color: Color,
}

impl Default for Subtitles {
    fn default() -> Self {
        Self {
            subtitle_bold: true,
            subtitle_color: DEFAULT_COLOR_OF_SUBTITLES,
            separator: Separator::default(),
            info_color: DEFAULT_COLOR_OF_INFOS,
        }
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            char: default_separator(),
            color: DEFAULT_COLOR_OF_SUBTITLES_SEPARATOR,
        }
    }
}
