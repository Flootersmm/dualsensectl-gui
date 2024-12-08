use chrono::prelude::*;
use std::process::Command;

pub fn move_test_file() {
    let current_time = Utc::now();
    let formatted_time = format!("test{}.txt", current_time.format("%Y-%m-%d_%H-%M-%S"));

    if let Err(err) = Command::new("sh")
        .arg("-c")
        .arg(format!("mv ./test.txt ./logs/{}", formatted_time))
        .output()
    {
        eprintln!("Failed to move file: {}", err);
    }
}
