use crate::config::{
    components::{
        cpu::CPU, de::DE, disk::Disk, gpu::GPU, image::Image, kernel::Kernel, memory::Memory,
        os::OS, resolution::Resolution, shell::Shell, subtitles::Subtitles, uptime::Uptime,
    },
    title::Title,
    util::{default_gap, default_true},
};

use serde::Deserialize;

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

impl Config {
    fn new(file: String) -> Result<Self, toml::de::Error> {
        toml::from_str::<Self>(file.as_str())
    }
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