use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Response {
    pub post_meta: PostMeta,
    pub title: Title,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostMeta {
    pub alert_style: String, // alert (there are more)
    pub site_id_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Title {
    pub rendered: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordPost {
    pub content: String,
}
