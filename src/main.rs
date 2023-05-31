mod mongo_developer;

use mp3::{can_open, check_mp3};
use std::env;

#[tokio::main]
async fn main() {
    // println!("=======> start check");
    // mongo_developer::check_size().await;
    // println!("=======> done check");

    let start_time = chrono::Local::now();
    // check mp3
    println!("=======> start mp3 check, start_time: {}", start_time.to_string());

    match env::current_dir() {
        Ok(current_dir) => {
            println!("Current directory: {:?}", current_dir);
        }
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    }

    let result = check_mp3().await;
    match result {
        Ok(_) => {
            println!("ok");
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }

    let end_time = chrono::Local::now();
    println!("=======> done mp3 check, start_time: {}", end_time.to_string());
}
