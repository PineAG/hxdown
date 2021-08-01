pub mod ehentai;

use super::fs::HxFS;
use crate::err::HxError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bytes::BufMut;
use tokio::time::{sleep, Duration};
use std::convert::TryFrom;

use select::document::Document;

pub struct CrawlerHelper<'a> {
    fs: &'a dyn HxFS
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

impl<'a> CrawlerHelper<'a> {
    pub fn new(fs: &'a dyn HxFS) -> Self {
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
        self.fs.mkdir(name).await?;
        Result::Ok(())
    }
    pub async fn finished(&self, name: &str) -> Result<bool, HxError> {
        let is_dir = self.fs.is_dir(name).await?;
        let has_meta = self.fs.is_file(&format!("{}/{}", name, "meta.json")).await?;
        Result::Ok(is_dir && has_meta)
    }
    pub async fn image_exists(&self, name: &str, page: i32) -> Result<bool, HxError> {
        let res = self.fs.is_file(&format!("{}/{:06}.jpg", name, page)).await?;
        Result::Ok(res)
    }
    pub async fn write_image(&self, name: &str, page: i32, buf: bytes::Bytes) -> Result<(), HxError> {
        let fp = format!("{}/{:06}.jpg", name, page);
        self.fs.write(&fp, buf).await?;
        Result::Ok(())
    }
    pub async fn save_meta(&self, meta: HxMeta) -> Result<(), HxError> {
        let fp = format!("{}/meta.json", meta.title);
        let json = serde_json::to_string(&meta).unwrap();
        let mut buf = bytes::BytesMut::with_capacity(json.len());
        let vec = json.into_bytes();
        let bytes: &[u8] = &vec;
        buf.put(bytes);
        self.fs.write(&fp, buf.freeze()).await?;
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