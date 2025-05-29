use crate::config::Config;
use crate::info::Info;
use colored::{Color, Colorize};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

pub fn print(config: Config, info: Info) {
    let image_lines = get_image_lines(config.image_source());

    match image_lines {
        Ok(image_lines) => {
            let mut image_lines: VecDeque<String> = image_lines.into();

            let titles_color = get_color(config.titles_color());
            let infos_color = get_color(config.infos_color());
            let image_color = get_color(config.image_color());
            let separator_color = get_color(config.separator_color());
            let gap = " ".repeat(config.gap().abs() as usize);

            let separator = ":".color(separator_color);

            let infos = vec![
                ("User", info.user()),
                ("OS", info.os()),
                ("Host", info.host()),
                ("Kernel", info.kernel()),
                ("Uptime", info.uptime()),
                ("Shell", info.shell()),
                ("Display", info.display()),
                ("CPU", info.cpu()),
                ("Memory", info.memory()),
                ("Local ip", Info::ip()),
                ("Battery", Info::battery()),
            ];

            println!(
                "{}",
                image_lines
                    .pop_front()
                    .unwrap_or("".to_string())
                    .color(image_color)
            );

            println!(
                "{}",
                image_lines
                    .pop_front()
                    .unwrap_or("".to_string())
                    .color(image_color)
            );

            for info in infos.iter() {
                println!(
                    "{}{}{}{} {}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color),
                    &gap,
                    info.0.color(titles_color).bold(),
                    &separator,
                    info.1.color(infos_color)
                );
            }

            for disk in info.disks() {
                println!(
                    "{}{}{}{} {}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color),
                    &gap,
                    "Disk".color(titles_color).bold(),
                    &separator,
                    disk.color(infos_color)
                );
            }

            if config.show_color_blocks() {
                println!(
                    "{}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color)
                );

                println!(
                    "{}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color)
                );

                println!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color),
                    &gap,
                    "   ".on_blue(),
                    "   ".on_green(),
                    "   ".on_cyan(),
                    "   ".on_red(),
                    "   ".on_purple(),
                    "   ".on_white(),
                    "   ".on_black(),
                    "   ".on_yellow()
                );

                println!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    image_lines
                        .pop_front()
                        .unwrap_or("".to_string())
                        .color(image_color),
                    &gap,
                    "   ".on_bright_blue(),
                    "   ".on_bright_green(),
                    "   ".on_bright_cyan(),
                    "   ".on_bright_red(),
                    "   ".on_bright_purple(),
                    "   ".on_bright_white(),
                    "   ".on_bright_black(),
                    "   ".on_bright_yellow()
                );
            }

            while !image_lines.is_empty() {
                println!("{}", image_lines.pop_front().unwrap().color(image_color))
            }
        }
        Err(e) => println!("{}", e.red()),
    }
}

fn get_color(str: &str) -> Color {
    match str {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "bright-black" => Color::BrightBlack,
        "bright-red" => Color::BrightRed,
        "bright-green" => Color::BrightGreen,
        "bright-yellow" => Color::BrightYellow,
        "bright-blue" => Color::BrightBlue,
        "bright-magenta" => Color::BrightMagenta,
        "bright-cyan" => Color::BrightCyan,
        "bright-white" => Color::BrightWhite,
        _ => Color::White,
    }
}

fn get_image_lines(path: &str) -> Result<Vec<String>, String> {
    let mut path = path;

    if path == "default" {
        println!("Default");
        path = &"/Users/jamshidelmurodov/.config/rfetch/default.txt";
    }

    let p = Path::new(path);

    if !p.exists() {
        return Err("Image file not found".to_string());
    }

    if let Some(ext) = p.extension() {
        if ext != "txt" {
            return Err("File can be only txt".to_string());
        }
    } else {
        return Err("File can be only txt".to_string());
    }

    match fs::read_to_string(path) {
        Ok(content) => {
            let lines = content.lines().map(|s| s.to_string()).collect();
            Ok(lines)
        }
        Err(_e) => Err("Error on processing image file".to_string()),
    }
}
