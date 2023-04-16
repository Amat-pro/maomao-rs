use mongodb::{options::ClientOptions, Client};
use std::sync::Once;
use futures::executor::block_on;

const URL: &str = "mongodb://maomao:maomao%40123@192.168.9.111:27017/Mytest";

static START: Once = Once::new();
// MONGO_CLIENT Do not edit it unless init method
static mut MONGO_CLIENT: Option<Client> = None;

pub fn get_client() -> Option<Client> {
    // init once
    START.call_once(|| init_mongo_client());
    unsafe {
        MONGO_CLIENT.clone()
    }
}

// pub fn get_client_v1() -> &'static Client {
//     // init once
//     START.call_once(|| init_mongo_client());
//     unsafe {
//         match &MONGO_CLIENT {
//             Some(client) => client,
//             None => panic!("MongoDB client is not initialized!"),
//         }
//     }
// }

fn init_mongo_client() {
    let r = block_on(ClientOptions::parse(URL));
    match r {
        Ok(opt) => unsafe {
            let client = Client::with_options(opt).unwrap();
            MONGO_CLIENT = Some(client)
        }
        Err(e) => {
            panic!("parse mongo options fail: err: {}", e)
        }
    }
    println!("init mongo client success!");
}