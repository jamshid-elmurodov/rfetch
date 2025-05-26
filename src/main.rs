mod args;
mod info;
mod output;
mod command;

const VERSION: &str = "1.0.0";

fn main() {
    command::command(args::parse());
}
