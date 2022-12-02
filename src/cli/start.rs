use std::fs::{create_dir_all, File};

use dirs::home_dir;

pub fn start() {
    println!("Start");
    let file_path = get_file_path();
    create_if_not_exist(file_path).expect("Failed to create file");
}

fn get_file_path() -> std::path::PathBuf {
    let home_dir = home_dir().expect("Failed to get home directory");
    let file_path = std::path::Path::new(&home_dir).join(".tmonitor/time.txt");
    file_path
}

fn create_if_not_exist(file_path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !file_path.exists() {
        println!("time.txt not found. Creating time.txt...");
        create_dir_all(file_path.parent().ok_or("Find parent dir failed")?)?;
        File::create(file_path)?;
    }
    Ok(())
}
