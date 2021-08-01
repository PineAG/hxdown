use super::{CrawlerHelper, NextPageStatus, HxMeta};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::{HashSet, HashMap};

pub struct EHentaiCrawler {
    
}

impl<'a> EHentaiCrawler {
    pub fn new() -> Self {
        return EHentaiCrawler{}
    }
    pub async fn download(&self, helper: &'a CrawlerHelper<'a>, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Cookie"), String::from("sl=dm_1; nw=1"));
        let doc_box = helper.get_page_with_headers(url, headers).await?;
        let doc: &Document = doc_box.as_ref();
        let meta = self.parse_meta(doc)?;
        println!("{}", meta.title);
        if helper.finished(&meta.title).await? {
            return Result::Ok(())
        }
        let first_page_url = doc.find(Class("gdtm").descendant(Name("a"))).next().expect("Cannot find 1st page.").attr("href").unwrap();
        let mut next_page: i32 = 0;
        let mut next_url: String = String::from(first_page_url);
        helper.create_dir(&meta.title).await?;
        loop {
            let res = self.download_page(helper, &meta.title, next_page, &next_url).await?;
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
    fn parse_meta(&self, doc: &Document) -> Result<HxMeta, Box<dyn std::error::Error>> {
        let title = doc.find(Attr("id", "gd2").descendant(Attr("id", "gn"))).next().expect("Title Element not found").text();
        let title_ja = doc.find(Attr("id", "gd2").descendant(Attr("id", "gj"))).next().expect("Title JA Element not found").text();
        let mut tags: HashSet<String> = HashSet::new();
        for ele in doc.find(Class("gtl")){
            tags.insert(ele.text());
        }
        Result::Ok(HxMeta{ title: title, title_ja: title_ja, tags: tags })
    }
    async fn download_page(&self, helper: &'a CrawlerHelper<'a>, name: &str, page: i32, url: &str) -> Result<NextPageStatus, Box<dyn std::error::Error>> {
        let doc_box = helper.get_page(url).await?;
        let doc: &Document = doc_box.as_ref();
        
        if !helper.image_exists(name, page).await? {
            let src = doc.find(Attr("id", "img")).next().expect("Failed to find image.").attr("src").unwrap();
            let data = helper.get_data(src).await?;
            helper.write_image(name, page, data).await?;
        }

        println!("Image: {} / {}", name, page);
        let next_page_url = doc.find(Attr("id", "next")).next().expect("No next button").attr("href").expect("URL href is empty");
        Result::Ok(if next_page_url == url {
            NextPageStatus::LastPage
        } else {
            NextPageStatus::HasNext{page: page+1, url: String::from(next_page_url)}
        })
    }
}
