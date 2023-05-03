use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    // post_meta: PostMeta,
    pub title: Title,
}

#[derive(Serialize, Deserialize, Clone)]
struct PostMeta {
    alert_style: String, // alert (there are more)
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Title {
    pub rendered: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordPost {
    pub content: String,
}
