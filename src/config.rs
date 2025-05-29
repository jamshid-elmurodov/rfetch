use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    #[serde(default = "title_default")]
    titles_color: String,
    #[serde(default = "infos_default")]
    infos_color: String,
    #[serde(default = "separator_default")]
    separator_color: String,
    #[serde(default = "image_source_default")]
    image_source: String,
    #[serde(default = "image_default")]
    image_color: String,
    #[serde(default = "gap_default")]
    gap: i32,
    #[serde(default = "default_true")]
    show_color_blocks: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            titles_color: title_default(),
            infos_color: infos_default(),
            separator_color: separator_default(),
            image_source: image_source_default(),
            image_color: image_default(),
            gap: gap_default(),
            show_color_blocks: true,
        }
    }
}

impl Config {
    pub fn new(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn titles_color(&self) -> &str {
        &self.titles_color
    }

    pub fn infos_color(&self) -> &str {
        &self.infos_color
    }

    pub fn separator_color(&self) -> &str {
        &self.separator_color
    }

    pub fn image_color(&self) -> &str {
        &self.image_color
    }

    pub fn gap(&self) -> i32 {
        self.gap
    }

    pub fn show_color_blocks(&self) -> bool {
        self.show_color_blocks
    }

    pub fn image_source(&self) -> &str {
        &self.image_source
    }
}

fn title_default() -> String {
    String::from("bright-yellow")
}

fn infos_default() -> String {
    String::from("white")
}

fn separator_default() -> String {
    String::from("white")
}

fn image_default() -> String {
    String::from("blue")
}

fn gap_default() -> i32 {
    2
}

fn default_true() -> bool {
    true
}

fn image_source_default() -> String {
    String::from(std::env::var("HOME").unwrap_or(String::from("")) + "/.config/rfetch/default.txt")
}
