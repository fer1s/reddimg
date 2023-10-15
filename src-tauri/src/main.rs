// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fetcher;

use std::fs;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").expect("$APPDATA not set!");
        [&appdata, "reddimg"].iter().collect()
    } else {
        let home = std::env::var("HOME").expect("$HOME not set!");
        [&home, ".config", "reddimg"].iter().collect()
    }
});

fn initialize_config() {
    // Create config directory if it doesn't exist
    if !CONFIG_DIR.exists() {
        fs::create_dir_all(&*CONFIG_DIR).expect("Unable to create config directory");
    }

    // Define config file path
    let config_file = CONFIG_DIR.join("config.json");
    println!("Config file path: {}", config_file.display());

    // Create config file if it doesn't exist
    if !config_file.exists() {
        let default_config = r#"{
            "subreddits": []
        }"#;

        fs::write(&config_file, default_config).expect("Unable to write config file");

        println!("Config file created at {}", config_file.display());

        return;
    }

    // Read config file
    let config = std::fs::read_to_string(&config_file).expect("Unable to read config file");

    // Parse config file
    let config: serde_json::Value =
        serde_json::from_str(&config).expect("Unable to parse config file");
    let config = Mutex::new(config);
    let config = config.lock().unwrap();

    println!("Config file loaded from {}", config_file.display());
    drop(config);
}

// Get subreddits from config (Return as JSON)
#[tauri::command]
fn get_subreddits() -> String {
    // Read config file
    let config_file = CONFIG_DIR.join("config.json");
    let config = std::fs::read_to_string(&config_file).expect("Unable to read config file");

    // Parse config file
    let config: serde_json::Value =
        serde_json::from_str(&config).expect("Unable to parse config file");
    let config = Mutex::new(config);
    let config = config.lock().unwrap();

    // Get subreddits from config
    let subreddits = config["subreddits"].as_array().unwrap();

    // Return subreddits as JSON
    serde_json::to_string(&subreddits).expect("Unable to convert subreddits to JSON")
}

// Add subreddit to config
#[tauri::command]
fn add_subreddit(subreddit: String) -> bool {
    // Read config file
    let config_file = CONFIG_DIR.join("config.json");
    let config = std::fs::read_to_string(&config_file).expect("Unable to read config file");

    // Parse config file
    let config: serde_json::Value =
        serde_json::from_str(&config).expect("Unable to parse config file");
    let config = Mutex::new(config);
    let mut config = config.lock().unwrap();

    // Get subreddits from config
    let subreddits = config["subreddits"].as_array_mut().unwrap();

    // Add subreddit to config
    if !subreddits.contains(&serde_json::Value::String(subreddit.clone())) {
        subreddits.push(serde_json::Value::String(subreddit.clone()));
    } else {
        return false;
    }

    // Write config file
    fs::write(&config_file, config.to_string()).expect("Unable to write config file");

    println!("Added subreddit {} to config", subreddit);

    true
}

// Remove subreddit from config
#[tauri::command]
fn remove_subreddit(subreddit: String) -> bool {
    // Read config file
    let config_file = CONFIG_DIR.join("config.json");
    let config = std::fs::read_to_string(&config_file).expect("Unable to read config file");

    // Parse config file
    let config: serde_json::Value =
        serde_json::from_str(&config).expect("Unable to parse config file");
    let config = Mutex::new(config);
    let mut config = config.lock().unwrap();

    // Get subreddits from config
    let subreddits = config["subreddits"].as_array_mut().unwrap();

    // Remove subreddit from config
    let mut subreddit_index = None;
    for (index, subreddit_item) in subreddits.iter().enumerate() {
        if subreddit_item.as_str().unwrap() == subreddit {
            subreddit_index = Some(index);
        }
    }

    if let Some(index) = subreddit_index {
        subreddits.remove(index);
    } else {
        return false;
    }

    // Write config file
    fs::write(&config_file, config.to_string()).expect("Unable to write config file");

    println!("Removed subreddit {} from config", subreddit);

    true
}

// Get images
#[tauri::command]
async fn get_images(quantity: u32) -> String {
    // Get subreddits from config and convert to Vec<&str>
    let subreddits = get_subreddits();
    let subreddits: Vec<&str> = serde_json::from_str(&subreddits).expect("Unable to parse subreddits");

    // Get images (returns Result<Vec<String>...>)
    let images = fetcher::fetch_x_images(subreddits, quantity).await;
    let images = images.unwrap();

    serde_json::to_string(&images).expect("Unable to convert images to JSON")
}

#[tokio::main]
async fn main() {
    // Initialize config
    initialize_config();

    // Temp file
    fetcher::reset_temp_file();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_subreddits,
            add_subreddit,
            remove_subreddit,
            get_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
