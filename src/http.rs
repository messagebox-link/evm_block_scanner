use std::io::{Error, ErrorKind};
use std::time::Duration;

use async_std::io;
use async_std::task::sleep;
use log::{debug, error};

use crate::{JsonRpc, JsonRpcRequest, random_number};

pub async fn http_send(urls: &Vec<String>, retry: u32, query: JsonRpcRequest) -> Option<JsonRpc> {
    let mut retry = retry;
    while retry.gt(&0) {
        let item = random_number(urls.len());
        if let Some(url) = urls.get(item) {
            debug!("🔧 Get random url: {:?}", url);

            if let Ok(req) = surf::post(url).body_json(&query) {
                // Async IO Timeout
                if let Ok(t) = io::timeout(Duration::from_secs(60), async {
                    if let Ok(mut res) = req.await {
                        if let Ok(content) = res.body_string().await {
                            match serde_json::from_str(content.as_str()) {
                                Ok(t) => { return Ok(t); }
                                Err(e) => { error!("{:?}", e) }
                            }
                        }
                    }
                    error!("🛸 Convert body");
                    Err(Error::new(ErrorKind::Other, "oh no!"))
                }).await {
                    return Some(t);
                }
            }
        }

        retry = retry - 1;
        sleep(Duration::from_secs(2)).await;
    }
    None
}