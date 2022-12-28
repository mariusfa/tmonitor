mod file_utils;
mod help;
mod start;
mod status;

#[derive(Debug, PartialEq)]
pub enum Action {
    Start,
    Status,
    Help,
    TooManyArgs,
}

pub fn parse_args(args: &Vec<String>) -> Action {
    if args.len() == 1 {
        return Action::Help;
    } else if args.len() == 2 {
        return match args[1].as_str() {
            "start" => Action::Start,
            "status" => Action::Status,
            _ => Action::Help,
        };
    } else {
        return Action::TooManyArgs;
    }
}

pub fn perform_action(action: Action) {
    match action {
        Action::Help => help::help(),
        Action::TooManyArgs => println!("Too many args"),
        Action::Start => start::start().expect("Failed to start"),
        Action::Status => status::status().expect("Failed to get status"),
    }
}
