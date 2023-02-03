use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    id: usize,
    date: String,
    link: String,
    modified: String,
    post_meta: PostMeta,
    pub title: Title,
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        return self.title.rendered == other.title.rendered;
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct PostMeta {
    alert_style: String, // alert (there are more)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Title {
    pub rendered: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordPost {
    pub content: String,
}