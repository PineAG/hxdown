use super::{CrawlerHelper, NextPageStatus, HxMeta};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::{HashSet};

pub struct NHentaiCrawler {
    
}

fn resolve_url(s: &str) -> String {
    let mut base = "https://nhentai.net".to_owned();
    base.push_str(s);
    base
}

impl NHentaiCrawler {
    pub fn new() -> Self {
        return NHentaiCrawler{}
    }
    fn parse_meta(&self, doc: &Document) -> Result<HxMeta, Box<dyn std::error::Error>> {
        let title = doc.find(Name("h1").and(Class("title"))).next().expect("Title Element not found").text();
        let title_ja = doc.find(Name("h2").and(Class("title"))).next().expect("Title JA Element not found").text();
        let mut tags: HashSet<String> = HashSet::new();
        for ele in doc.find(Class("tag").descendant(Class("name"))){
            tags.insert(ele.text());
        }
        Result::Ok(HxMeta{ title: title, title_ja: title_ja, tags: tags })
    }
    async fn download_page(&self, helper: &CrawlerHelper, name: &str, page: i32, url: &str) -> Result<NextPageStatus, Box<dyn std::error::Error>> {
        let doc_box = helper.get_page(url).await?;
        let doc: &Document = doc_box.as_ref();
        
        if !helper.image_exists(name, page).await? {
            let src = doc.find(Attr("id", "image-container").descendant(Name("a")).descendant(Name("img"))).next().expect("Failed to find image.").attr("src").unwrap();
            let data = helper.get_data(src).await?;
            helper.write_image(name, page, data).await?;
        }

        println!("Image: {} / {}", name, page);
        Ok(match doc.find(Class("next").and(Class("invisible").not())).next() {
            Some(ele) => {
                let next_page_url = ele.attr("href").expect("href not found");
                let next_page_url = resolve_url(next_page_url);
                NextPageStatus::HasNext{page: page+1, url: String::from(next_page_url)}
            },
            None => NextPageStatus::LastPage
        })
    }

    pub async fn download(&self, helper: &CrawlerHelper, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let doc_box = helper.get_page(url).await?;
        let doc: &Document = &doc_box;
        let meta = self.parse_meta(doc)?;
        println!("{}", meta.title);
        if helper.finished(&meta.title).await? {
            return Result::Ok(())
        }
        let first_page_url = doc.find(Class("gallerythumb")).next().expect("Cannot find 1st page.").attr("href").unwrap();
        let first_page_url = resolve_url(first_page_url);
        let mut next_page: i32 = 0;
        let mut next_url: String = String::from(first_page_url);
        helper.create_dir(&meta.title).await?;
        loop {
            let res = self.download_page(&helper, &meta.title, next_page, &next_url).await?;
            match res {
                NextPageStatus::HasNext{page, url} => {
                    next_page = page;
                    next_url = String::from(url);
                },
                NextPageStatus::LastPage => break
            }
        }
        helper.save_meta(meta).await?;
        Result::Ok(())
    }
}

