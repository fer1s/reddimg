// Fetcher module by fer1s

use async_recursion::async_recursion;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use std::sync::Mutex;
use std::{fs, time::Duration};
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditImage {
    pub subreddit: String,
    pub title: String,
    pub post_url: String,
    pub image_url: String,
}

// TEMP
pub fn reset_temp_file() {
    // CONFIG_DIR
    let temp_file = crate::CONFIG_DIR.join("temp.json");
    let default_temp = r#"{
        "ids": []
    }"#;

    // Check if file exists, delete if it does
    if temp_file.exists() {
        std::fs::remove_file(&temp_file).expect("Unable to delete temp file");
    }

    // Create temp file
    fs::write(&temp_file, default_temp).expect("Unable to write temp file");

    println!("Temp file created at {}", temp_file.display());
}

pub fn push_temp_id(id: &str) {
    // Check if id is in temp
    let temp_file = crate::CONFIG_DIR.join("temp.json");
    let temp = std::fs::read_to_string(&temp_file).expect("Unable to read temp file");

    // Parse temp file
    let temp: serde_json::Value = serde_json::from_str(&temp).expect("Unable to parse temp file");
    let temp = Mutex::new(temp);
    let mut temp = temp.lock().unwrap();

    // Get ids from temp
    let ids = temp["ids"].as_array_mut().unwrap();

    // Push id to ids
    ids.push(serde_json::Value::String(id.to_string()));

    // Write temp file
    fs::write(&temp_file, temp.to_string()).expect("Unable to write temp file");
}

pub fn check_temp_id(id: &str) -> bool {
    // Check if id is in temp
    let temp_file = crate::CONFIG_DIR.join("temp.json");
    let temp = std::fs::read_to_string(&temp_file).expect("Unable to read temp file");

    // Parse temp file
    let temp: serde_json::Value = serde_json::from_str(&temp).expect("Unable to parse temp file");
    let temp = Mutex::new(temp);
    let temp = temp.lock().unwrap();

    // Get ids from temp
    let ids = temp["ids"].as_array().unwrap();

    // Check if id is in ids
    let mut is_in_ids = false;
    for i in ids {
        if i.as_str().unwrap() == id {
            is_in_ids = true;
        }
    }

    // If id is in ids, return true
    if is_in_ids {
        return true;
    } else {
        return false;
    }
}

pub async fn fetch_image(subreddit: &str) -> Result<RedditImage, Box<dyn Error>> {
    fetch_image_recursive(subreddit).await
}

#[async_recursion]
async fn fetch_image_recursive(subreddit: &str) -> Result<RedditImage, Box<dyn Error>> {
    // sleep(Duration::from_millis(500)).await;
    println!("Fetching image from {}", subreddit);
    // Create reqwest client
    let client = Client::new();
    // Get random post from subreddit with an image or gif and return it (limit=1)
    let url = format!("https://reddit.com/r/{}/random/.json?limit=1", subreddit);

    let res = client
        .get(&url)
        .header("User-Agent", "reddimg/0.0.1")
        .send()
        .await;
    let res = match res {
        Ok(res) => res,
        Err(_) => return fetch_image_recursive(subreddit).await,
    };

    // If 429, wait 5 seconds and try again (429 Too Many Requests)
    if res.status().as_u16() == 429 {
        println!("429 Too Many Requests, waiting 5 seconds");
        sleep(Duration::from_secs(5)).await;
        return fetch_image_recursive(subreddit).await;
    }

    let res = res.json::<Value>().await?;

    let post = &res[0]["data"]["children"][0]["data"];

    let image_url = post["url"].as_str();
    let image_url = match image_url {
        Some(image_url) => image_url,
        None => return fetch_image_recursive(subreddit).await,
    };

    // Check if the image is an image or gif
    if !image_url.ends_with(".jpg")
        && !image_url.ends_with(".png")
        && !image_url.ends_with(".gif")
        && !image_url.ends_with(".jpeg")
    {
        return fetch_image_recursive(subreddit).await;
    }

    //let subreddit_id = post["subreddit_id"].as_str().unwrap();
    let post_id = post["id"].as_str();
    let post_id = match post_id {
        Some(post_id) => post_id,
        None => return fetch_image_recursive(subreddit).await,
    };
    // Check if the post has already been posted
    if check_temp_id(post_id) {
        return fetch_image_recursive(subreddit).await;
    }

    let post_url = format!("https://reddit.com/r/{}/comments/{}", subreddit, post_id);
    let title = post["title"].as_str();
    let title = match title {
        Some(title) => title,
        None => return fetch_image_recursive(subreddit).await,
    };

    // Push post id to temp
    push_temp_id(post_id);

    Ok(RedditImage {
        subreddit: subreddit.to_string(),
        title: title.to_string(),
        post_url: post_url.to_string(),
        image_url: image_url.to_string(),
    })
}

pub async fn fetch_x_images(
    subreddits: Vec<&str>,
    x: u32,
) -> Result<Vec<RedditImage>, Box<dyn std::error::Error>> {
    // X is the amount of images to fetch
    let mut images: Vec<RedditImage> = Vec::new();

    for _ in 0..x {
        // Get random subreddit from subreddits
        let subreddit = subreddits.choose(&mut rand::thread_rng()).unwrap();

        // Fetch image from subreddit
        println!("X|Fetching image from {}", subreddit);
        let image = fetch_image(subreddit).await?;

        images.push(image);
    }

    Ok(images)
}
