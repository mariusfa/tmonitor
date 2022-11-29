use std::env;

mod help;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() == 1 {
        help::help();
    } else if args.len() == 2 {
        print!("Do actions");
    } else {
        print!("Too many args");
    }
}
