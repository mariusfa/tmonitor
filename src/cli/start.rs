use chrono::{Local, TimeZone};
use std::{
    fs::{self, create_dir_all, File},
    thread,
    time
};

use crate::{result::ResultWrap, cli::file_utils};

pub fn start() -> ResultWrap<()> {
    println!("Start");
    
    let file_path = file_utils::get_file_path()?;
    create_if_not_exist(&file_path)?;

    loop {
        let contents = fs::read_to_string(&file_path)?;
    
        if contents.len() == 0 {
            set_first_timestamp(&file_path);
        } else {
            let mut lines: Vec<&str> = contents.split("\n").collect();
            let last_line_index = lines.len() - 1;
            let (first, second) = extract_timstamps_from_line(lines[last_line_index]);
    
            let now_timestamp = Local::now().timestamp();
    
            if is_today(second, now_timestamp) {
                let updated_line = create_line(first, now_timestamp.to_string().as_str());
                lines[last_line_index] = updated_line.as_str();
                fs::write(&file_path, lines.join("\n"))?;
            } else {
                let new_line = create_line(
                    now_timestamp.to_string().as_str(),
                    now_timestamp.to_string().as_str(),
                );
                lines.push(new_line.as_str());
                fs::write(&file_path, lines.join("\n"))?;
            }
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn create_if_not_exist(file_path: &std::path::PathBuf) -> ResultWrap<()> {
    if !file_path.exists() {
        println!("time.txt not found. Creating time.txt...");
        create_dir_all(file_path.parent().ok_or("Find parent dir failed")?)?;
        File::create(file_path)?;
    } else {
        println!("time.txt found. Updating time.txt...");
    }
    Ok(())
}

fn set_first_timestamp(file_path: &std::path::PathBuf) {
    let timestamp = Local::now().timestamp();
    fs::write(file_path, format!("{timestamp} {timestamp}")).expect("oops")
}

fn extract_timstamps_from_line(line: &str) -> (&str, &str) {
    let timestamps_last_line: Vec<&str> = line.split_whitespace().collect();
    let first = timestamps_last_line[0];
    let second = timestamps_last_line[1];
    return (first, second);
}

fn is_today(last_timestamp: &str, now_timestamp: i64) -> bool {
    let last_timestamp: i64 = last_timestamp.parse().unwrap();
    let last_date = Local
        .timestamp_opt(last_timestamp, 0)
        .unwrap()
        .format("%d %m %Y")
        .to_string();

    let now_date = Local
        .timestamp_opt(now_timestamp, 0)
        .single()
        .unwrap()
        .format("%d %m %Y")
        .to_string();

    return last_date == now_date;
}

fn create_line(first: &str, second: &str) -> String {
    let mut line = String::new();
    line.push_str(first);
    line.push_str(" ");
    line.push_str(second);
    return line;
}
