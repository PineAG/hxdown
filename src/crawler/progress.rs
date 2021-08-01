use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use super::HxMeta;


#[derive(Serialize, Deserialize)]
pub struct HxStatus {
    status: HashMap<String, HxCrawlerStatus>,
    queue: VecDeque<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HxCrawlerStatus {
    Downloading{
        url: String,
        meta: HxMeta,
        pages: HashMap<i32, HxCrawlerPage>
    },
    Done,
    Dead{
        url: String,
        meta: HxMeta,
        pages: HashMap<i32, HxCrawlerPage>
    },
}

#[derive(Serialize, Deserialize)]
pub struct HxCrawlerPage {
    url: String,
    next_url: String
}
