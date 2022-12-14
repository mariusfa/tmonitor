use std::env;

mod cli;
mod result;

fn main() {
    let args: Vec<String> = env::args().collect();

    let action = cli::parse_args(&args);
    cli::perform_action(action);
}
