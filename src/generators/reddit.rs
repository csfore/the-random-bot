//! Description: Function to get Reddit post

use rand::prelude::SliceRandom;
use roux::Subreddit;
use crate::generators::structs::{PostData, RawPost};

pub async fn get_post() -> PostData {
    let subreddit = Subreddit::new("all");

    // Get new posts with limit = 5
    let get_new = subreddit.latest(5, None).await;

    let jobj = serde_json::to_string(&get_new.unwrap());
    let post: RawPost = serde_json::from_str(&jobj.unwrap()).unwrap();
    //    for posts in post.data.children {
    //        println!("{:#?}", posts.data);
    //    }
    let rand_post = post.data.children.choose(&mut rand::thread_rng()).unwrap();

    let newobj = serde_json::to_string(&rand_post.data);
    let owned_post: PostData = serde_json::from_str(&newobj.unwrap()).unwrap();
    return owned_post
}