use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
struct MongoFeed12 {
    _id: i64,
    feed_id: i64,
    feed_title: String,
    collect_id: i64,

    video: Video,
    #[serde(default)]
    mini_video_url: String,
    #[serde(default)]
    mini_video_url_size: i64,
    #[serde(default)]
    duanju_video_url: String,
    #[serde(default)]
    duanju_video_url_size: i64,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Video {
    #[serde(default)]
    image_url: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    width: i64,
    #[serde(default)]
    height: i64,
    #[serde(default)]
    duration: String,
}