use core::time;
use std::fs::{self, create_dir_all, File};

use chrono::{Local, TimeZone};
use dirs::home_dir;

pub fn start() {
    println!("Start");
    let file_path = get_file_path();
    create_if_not_exist(&file_path).expect("Failed to create file");

    let contents = fs::read_to_string(&file_path).expect("read failed");

    if contents.len() == 0 {
        set_first_timestamp(file_path);
    } else {
        let mut lines: Vec<&str> = contents.split("\n").collect();
        let lines_length = lines.len();

        let mut last_line = lines[lines_length - 1];
        let timestamps_last_line: Vec<&str> = last_line.split_whitespace().collect();
        let first_timestamp = timestamps_last_line[0];
        let last_timestamp = timestamps_last_line[1];

        let now_timestamp = Local::now().timestamp();
        let now_date = Local
            .timestamp_opt(now_timestamp, 0)
            .single()
            .unwrap()
            .format("%d %m %Y")
            .to_string();

        let last_timestamp: i64 = last_timestamp.parse().unwrap();
        let last_date = Local
            .timestamp_opt(last_timestamp, 0)
            .unwrap()
            .format("%d %m %Y")
            .to_string();

        if last_date == now_date {
            let mut updated_line: String = first_timestamp.to_string();
            updated_line.push_str(" ");
            updated_line.push_str(now_timestamp.to_string().as_str());

            last_line = updated_line.as_str();

            lines[lines_length - 1] = last_line;
            let updated = lines.join("\n");
            fs::write(file_path, updated).expect("oops");
        } else {
            let timestamp = Local::now().timestamp().to_string();
            let timestamp = timestamp.as_str();
            let mut new_line = String::new();
            new_line.push_str(timestamp);
            new_line.push_str(" ");
            new_line.push_str(timestamp);
            lines.push(new_line.as_str());
            let updated = lines.join("\n");
            fs::write(file_path, updated).expect("oops");
        }
    }
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
