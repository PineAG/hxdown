pub mod ehentai;
pub mod nhentai;

use super::fs::HxFS;
use crate::err::HxError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bytes::BufMut;
use tokio::time::{sleep, Duration};
use std::convert::TryFrom;
use async_trait::async_trait;
use regex::Regex;

use select::document::Document;

#[async_trait]
pub trait HxCrawler {
    async fn download(&self, helper: &CrawlerHelper, url: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct CrawlerHelper {
    fs: Box<dyn HxFS + Send>
}

fn to_safe_str(path: &str) -> String {
    let safe_path_re = Regex::new("[<>:\"/\\|?*]").unwrap();
    let path = String::from(safe_path_re.replace_all(path, ""));
    path
}

async fn retry_request<F, Fut>(func: F) -> reqwest::Response 
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = reqwest::Result<reqwest::Response>> {
    for _ in 0..30 {
        let r = func().await;
        match r {
            Ok(val) => return val,
            Err(err) => {
                println!("Err: {}", err)
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
    panic!("Failed for too many times");
}

impl CrawlerHelper {
    pub fn new(fs: Box<dyn HxFS + Send>) -> Self {
        return CrawlerHelper {fs: fs}
    }
    pub async fn get_page(&self, url: &str) -> Result<Box<Document>, Box<dyn std::error::Error>> {
        let res = retry_request(|| { reqwest::get(url) }).await;
        return self.on_page_body(res).await;
    }
    pub async fn get_page_with_headers(&self, url: &str, headers: HashMap<String, String>) -> Result<Box<Document>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;
        let res = retry_request(|| { 
            let real_headers = http::HeaderMap::try_from(&headers).expect("Invalid headers");
            let req = client.get(url).headers(real_headers);
            req.send()
         }).await;
        return self.on_page_body(res).await;
    }
    async fn on_page_body(&self, res: reqwest::Response) -> Result<Box<Document>, Box<dyn std::error::Error>> {
        let html = res.text().await?;
        let html_str: &str = &html;
        let doc = Document::from(html_str);
        let doc_box = Box::new(doc);
        return Result::Ok(doc_box);
    }
    pub async fn get_data(&self, url: &str) -> Result<bytes::Bytes, Box<dyn std::error::Error>>  {
        let res: reqwest::Response = retry_request(|| {reqwest::get(url)}).await;
        let bytes = res.bytes().await?;
        Result::Ok(bytes)
    }

    pub async fn create_dir(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let safe_dir = to_safe_str(name);
        self.fs.mkdir(&safe_dir).await?;
        Result::Ok(())
    }
    pub async fn finished(&self, name: &str) -> Result<bool, HxError> {
        let safe_dir = to_safe_str(name);
        let is_dir = self.fs.is_dir(name).await?;
        let has_meta = self.fs.is_file(&safe_dir, &format!("{}/{}", name, "meta.json")).await?;
        Result::Ok(is_dir && has_meta)
    }
    pub async fn image_exists(&self, name: &str, page: i32) -> Result<bool, HxError> {
        let safe_dir = to_safe_str(name);
        let res = self.fs.is_file(&safe_dir, &format!("{:06}.jpg", page)).await?;
        Result::Ok(res)
    }
    pub async fn write_image(&self, name: &str, page: i32, buf: bytes::Bytes) -> Result<(), HxError> {
        let safe_dir = to_safe_str(name);
        let fp = format!("{:06}.jpg", page);
        self.fs.write(&safe_dir, &fp, buf).await?;
        Result::Ok(())
    }
    pub async fn save_meta(&self, meta: HxMeta) -> Result<(), HxError> {
        let safe_dir = to_safe_str(&meta.title);
        let fp = "meta.json";
        let json = serde_json::to_string(&meta).unwrap();
        let mut buf = bytes::BytesMut::with_capacity(json.len());
        let vec = json.into_bytes();
        let bytes: &[u8] = &vec;
        buf.put(bytes);
        self.fs.write(&safe_dir, &fp, buf.freeze()).await?;
        Result::Ok(())
    }
}

pub enum NextPageStatus {
    HasNext{page: i32, url: String},
    LastPage
}

#[derive(Serialize, Deserialize)]
pub struct HxMeta {
    title: String,
    title_ja: String,
    tags: std::collections::HashSet<String>
}
