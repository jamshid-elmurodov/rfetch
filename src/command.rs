use colored::Colorize;
use crate::{info, output, VERSION};

pub fn command(args: Vec<String>){
    if args.len() > 3 {
        println!("{}\n", "An error occurred.".red());
        print_help();
        return;
    }

    if args.len() == 1 {
        output::print(info::Info::get_info(), String::new());
    } else if args[1] == "version" {
        print_version();
    } else if args[1] == "help" {
        print_help()
    } else if args[1] == "ascii" {
        if args.len() < 3 {
            println!("{}", "Path is invalid, please give valid path to your ascii txt file".red());
            return;
        }

        output::print(info::Info::get_info(), args[2].clone());
    }
}

fn print_version() {
    println!("{} - {}", "Rfetch".blue(), VERSION);
}

fn print_help() {
    println!(
        "{} - {}\nUsage:\n   rfetch [options]\n\nOptions:\n   version                   Show version information\n   help                      Show help information\n   ascii <PATH_TO_FILE>      Set custom ascii art",
        "Rfetch".blue(),
        VERSION
    )
}