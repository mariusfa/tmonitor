use std::fs;

use chrono::{Local, TimeZone};

use crate::{cli::file_utils, result::ResultWrap};

pub fn status() -> ResultWrap<()> {
    println!("Status");
    let file_path = file_utils::get_file_path()?;
    if !file_path.exists() {
        println!("File not found. Try to run start to create file!");
        return Ok(());
    }
    let contents = fs::read_to_string(&file_path)?;

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut total_secs: i64 = 0;

    for line in lines.iter() {
        let (start, end) = file_utils::extract_timstamps_from_line(line);
        let start: i64 = start.parse()?;
        let end: i64 = end.parse()?;

        let start_date = get_date(start)?;

        let diff_sec = end - start;
        let diff_hour = (diff_sec as f64) / 3600.0;
        total_secs += diff_sec;

        println!("{start_date}  -  {diff_hour:.2}h");
    }
    let total_hours = (total_secs as f64) / 3600.0;
    println!("Total time: {total_hours:.2}h");
    Ok(())
}

fn get_date(timestamp: i64) -> ResultWrap<String> {
    Ok(Local
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or("Failed to convert to date")?
        .format("%d.%m.%Y")
        .to_string())
}
