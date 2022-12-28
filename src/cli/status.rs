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

        let start_date = get_date(start);
        let end_date = get_date(end);

        let diff_sec  = end - start;
        let diff_hour = (diff_sec as f64) / 3600.0;
        total_secs += diff_sec;
        

        println!("{start_date} -- {end_date}  -  {diff_hour}h");
    }
    let total_hours = (total_secs as f64) / 3600.0;
    println!("Total time: {total_hours}h");
    Ok(())
}

fn get_date(timestamp: i64) -> String {
    return Local
        .timestamp_opt(timestamp, 0)
        .unwrap()
        .format("%d.%m.%Y")
        .to_string();
}
