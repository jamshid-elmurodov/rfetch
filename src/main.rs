mod info;
mod output;
mod config;

use std::fs;
use colored::Colorize;
use crate::config::Config;

fn main() {
    let path = std::env::var("HOME").unwrap_or(String::new()) + "/.config/rfetch/config.toml";
    let content = fs::read_to_string(&path);

    let config;
    if content.is_err() {
        println!("{} {}\n\n", "Started using defaults because config file not found in".red(), path.yellow());
        config = Ok(Config::default());
    } else {
        let content= content.unwrap().to_string();
        config = Config::new(&content);
    }
    
    if config.is_err() {
        println!("{}", "Syntax error in config file".red());
        return;
    }
    
    let info = info::Info::new();
    output::print(config.unwrap(), info);
}
