use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    by: String,
    // descendants: i32,
    // id: i32,
    // "kids": Vec<i32>,
    // score: i32,
    // time: u32,
    title: String,
    // r#type: String,
    url: Option<String>,
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let uri_topstories = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
    let uri_post = "https://hacker-news.firebaseio.com/v0/item/";

    println!("Processing...");
    let top_stories = reqwest::get(uri_topstories)
        .await?
        .json::<Vec<u32>>()
        .await?;

    println!("{} New Posts", top_stories.len());

    let mut posts = Vec::new();

    for (key, item) in top_stories.iter().enumerate() {
        if key < 30 {
            let uri_item = format!("{}{}.json?print=pretty", uri_post, &item);
            let post = reqwest::get(uri_item.as_str())
                .await?
                .json::<Post>()
                .await?;
            let post_url: String = match &post.url {
                Some(url) => url.to_string(),
                None => "".to_string(),
            };
            println!(
                "{{
Post: {}
By: {}
Url: {}
}}",
                &post.title, &post.by, &post_url
            );
            posts.push(post);
        }
    }

    Ok(())
}

fn main() {
    match start() {
        Ok(()) => (),
        Err(err) => println!("{:?}", err),
    }
}
