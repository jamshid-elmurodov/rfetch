use serde::Deserialize;

const DEFAULT_COLOR_OF_TITLE: Color = Color::Blue;
const DEFAULT_COLOR_OF_UNDERLINE: Color = Color::Blue;
const DEFAULT_COLOR_OF_TITLES_SEPARATOR: Color = Color::Blue;
const DEFAULT_COLOR_OF_SUBTITLES: Color = Color::BrightYellow;
const DEFAULT_COLOR_OF_SUBTITLES_SEPARATOR: Color = Color::BrightYellow;
const DEFAULT_COLOR_OF_INFOS: Color = Color::White;
const DEFAULT_COLOR_OF_IMAGE: Color = Color::Blue;

fn default_true() -> bool { true }
fn default_gap() -> i32 { 2 }
fn default_separator() -> char { ':' }
fn default_source() -> String { "default".to_string() }

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Config {
    title: Title,
    os: OS,
    #[serde(default = "default_true")]
    show_host: bool,
    kernel_shorthand: Kernel,
    uptime: Uptime,
    shell: Shell,
    resolution: Resolution,
    #[serde(default = "default_true")]
    show_local_ip: bool,
    cpu: CPU,
    gpu: GPU,
    memory: Memory,
    de: DE,
    disk: Disk,
    subtitles: Subtitles,
    #[serde(default = "default_true")]
    show_color_blocks: bool,
    image: Image,
    #[serde(default = "default_gap")]
    gap: i32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Title {
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

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Kernel {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    shorthand: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct OS {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_os_arch: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Uptime {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    shorthand: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Shell {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_path: bool,
    #[serde(default = "default_true")]
    show_version: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CPU {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_brand: bool,
    #[serde(default = "default_true")]
    show_cores: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct GPU {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_brand: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Resolution {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_refresh_rate: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct DE {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_version: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Disk {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_percent: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Subtitles {
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

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Image {
    color: Color,
    #[serde(default = "default_source")]
    source: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Memory {
    #[serde(default = "default_true")]
    show: bool,
    #[serde(default = "default_true")]
    show_percent: bool,
    unit: Unit,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: Title::default(),
            os: OS::default(),
            show_host: true,
            kernel_shorthand: Kernel::default(),
            uptime: Uptime::default(),
            shell: Shell::default(),
            resolution: Resolution::default(),
            show_local_ip: true,
            cpu: CPU::default(),
            gpu: GPU::default(),
            memory: Memory::default(),
            de: DE::default(),
            disk: Disk::default(),
            subtitles: Subtitles::default(),
            show_color_blocks: true,
            image: Image::default(),
            gap: 2,
        }
    }
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

impl Default for Kernel {
    fn default() -> Self {
        Self {
            show: true,
            shorthand: false,
        }
    }
}

impl Default for OS {
    fn default() -> Self {
        Self {
            show: true,
            show_os_arch: true,
        }
    }
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            show: true,
            shorthand: false,
        }
    }
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

impl Default for CPU {
    fn default() -> Self {
        Self {
            show: true,
            show_brand: true,
            show_cores: true,
        }
    }
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            show: true,
            show_brand: true,
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            show: true,
            show_refresh_rate: true,
        }
    }
}

impl Default for DE {
    fn default() -> Self {
        Self {
            show: true,
            show_version: true,
        }
    }
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            show: true,
            show_percent: true,
        }
    }
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

impl Default for Image {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR_OF_IMAGE,
            source: default_source(),
        }
    }
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

#[derive(Debug, Deserialize)]
enum Unit {
    KiB,
    MiB,
    GiB,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::GiB
    }
}

#[derive(Debug, Deserialize)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Default for Color {
    fn default() -> Self {
        Color::Blue
    }
}