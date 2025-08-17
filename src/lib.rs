use std::fs;
use std::io::Write;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use std::thread::sleep;

/// Scrapes for new posts or videos by the Vegan Teacher and alerts you.
pub fn check_for_new_content() {
    println!("Checking for new Vegan Teacher content... 🕵️‍♂️");

    let client = Client::new();
    let url = "https://www.tiktok.com/@veganteacher"; // Replace with the actual URL if needed
    let response = client.get(url).send().expect("Failed to get response");
    let body = response.text().expect("Failed to get body text");
    let document = Html::parse_document(&body);

    // This selector might need to be adjusted based on TikTok's ever-changing HTML structure.
    let video_selector = Selector::parse("div.tiktok-video-item-selector").unwrap();

    let mut new_videos = Vec::new();

    for element in document.select(&video_selector) {
        let video_url = element.select(&Selector::parse("a").unwrap()).next().unwrap().value().attr("href").unwrap();
        let video_name = element.select(&Selector::parse("p.video-title").unwrap()).next().unwrap().text().collect::<String>();
        
        let existing_videos_content = fs::read_to_string("seen_videos.txt").unwrap_or_else(|_| String::new());
        if !existing_videos_content.contains(video_url) {
            new_videos.push((video_url.to_string(), video_name.to_string()));
            let mut file = fs::OpenOptions::new().append(true).create(true).open("seen_videos.txt").unwrap();
            writeln!(file, "{}", video_url).unwrap();
        }
    }

    if new_videos.is_empty() {
        println!("No new content found. Time to chill! 😌");
    } else {
        for (url, name) in new_videos {
            let now = chrono::Local::now();
            println!("🚨🚨 ALERT! Vegan teacher made content! Roast Now! 🚨🚨");
            println!("Video: {} released at {}", name, now.format("%d/%m/%y-%H:%M"));
            println!("Link: {}", url);
        }
    }
}

/// Scrapes for mentions of the Vegan Teacher in public comments (simulated).
pub fn check_for_mentions() {
    println!("Searching for mentions... 👀");

    // This part is a simplified simulation. Real-world scraping would be much more complex and may get blocked.
    // We'll simulate finding a mention.
    let simulated_mentions = vec!["@VeganTeacher", "VeganTeacher", "That vegan lady"];
    let sample_comment = "I can't believe @VeganTeacher said that!";
    
    for mention in &simulated_mentions {
        if sample_comment.contains(mention) {
            println!("📣📣 ALERT! Someone mentioned the Vegan Teacher! 📣📣");
            println!("Time to roast! Found mention: {}", mention);
        }
    }
}

// Just a main function for testing the library
fn main() {
    loop {
        check_for_new_content();
        check_for_mentions();
        // Wait for a few minutes before checking again to not overload servers
        println!("Sleeping for 5 minutes... 😴");
        sleep(Duration::from_secs(300)); 
    }
}