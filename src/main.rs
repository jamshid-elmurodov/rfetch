mod args;
mod info;
mod output;
mod command;
mod config;

const VERSION: &str = "1.0.0";

fn main() {
    command::command(args::parse());
}
