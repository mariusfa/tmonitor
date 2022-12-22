use std::fs::{self, create_dir_all, File};

use chrono::{Local, TimeZone};
use dirs::home_dir;

pub fn start() {
    println!("Start");
    let file_path = get_file_path();
    create_if_not_exist(&file_path).expect("Failed to create file");

    let mut contents = fs::read_to_string(&file_path).expect("read failed");

    if contents.len() == 0 {
        set_first_timestamp(file_path);
    } else {
        fs::write(file_path, format!("fuck")).expect("oops")
    }

    //     println!("{contents}");
    //     let mut timestamp_entries: Vec<&str> = contents.split("\n").collect();
    //     let num_timestamp_entries = timestamp_entries.len();

    //     let now = Local::now().timestamp();
    //     let now_date = Local
    //         .timestamp_opt(now, 0)
    //         .single()
    //         .unwrap()
    //         .format("%d %m %Y")
    //         .to_string();

    //     let start_time = timestamp_entries
    //         .last()
    //         .unwrap()
    //         .split_whitespace()
    //         .next()
    //         .unwrap();

    //     let last_entry = timestamp_entries
    //         .last()
    //         .unwrap()
    //         .split_whitespace()
    //         .last()
    //         .unwrap();

    //     let last_entry: i64 = last_entry.parse().unwrap();

    //     let last_date = Local
    //         .timestamp_opt(last_entry, 0)
    //         .single()
    //         .unwrap()
    //         .format("%d %m %Y")
    //         .to_string();

    //     println!("{now_date}");
    //     println!("{last_date}");
    //     let is_equal = now_date == last_date;
    //     println!("{is_equal}");

    //     if is_equal {
    //         println!("{start_time} {now}");
    //         timestamp_entries[num_timestamp_entries - 1] = format!("{start_time} {now}");
    //     } else {
    //     }

    //     let a = timestamp_entries.join("\n");
    //     //println!("{num_timestamp_entries}")
}

fn get_file_path() -> std::path::PathBuf {
    let home_dir = home_dir().expect("Failed to get home directory");
    let file_path = std::path::Path::new(&home_dir).join(".tmonitor/time.txt");
    file_path
}

fn create_if_not_exist(file_path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !file_path.exists() {
        println!("time.txt not found. Creating time.txt...");
        create_dir_all(file_path.parent().ok_or("Find parent dir failed")?)?;
        File::create(file_path)?;
    }
    Ok(())
}

fn set_first_timestamp(file_path: std::path::PathBuf) {
    let timestamp = Local::now().timestamp();
    fs::write(file_path, format!("{timestamp} {timestamp}")).expect("oops")
}
