use futures::{StreamExt};
use mongodb::bson::doc;
use crate::mongo_developer::mongo_client::get_client;
use serde::{Deserialize, Serialize};
use futures::future;

#[derive(Deserialize, Serialize, Default, Debug)]
struct MongoFeed101 {
    _id: i64,
    #[serde(default)]
    origin_url: String,
    #[serde(default)]
    f_size: i64,
    #[serde(default)]
    origin_content_length: i64,
}

pub async fn check_size() {
    let mongo_client = get_client().unwrap();
    let collection = mongo_client.
        database("Mytest").
        collection::<MongoFeed101>("mongo_feed_10");

    let filter = doc! {};
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let r = collection.find(filter, None).await;
    match r {
        Ok(cursor) => {
            let feeds = cursor.map(|doc| {
                doc.unwrap()
            }).filter(|feed| {
                if feed.origin_url.is_empty() {
                    return future::ready(false);
                }

                if feed.f_size < feed.origin_content_length {
                    return future::ready(true);
                }
                return future::ready(false);
            }).collect::<Vec<MongoFeed101>>().await;

            println!("feeds length: {}", feeds.len());
            for feed in feeds {
                let filter = doc! {
                    "_id": feed._id,
                };
                let update = doc! {
                    "$set": { "size_ne": true }
                };
                let _ = collection.update_one(filter, update, None).await;
            }
        }
        Err(e) => {
            panic!("fetch docs fail: {}", e)
        }
    }
}