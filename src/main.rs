mod fs;
mod crawler;
mod err;

use crawler::CrawlerHelper;
use fs::local::LocalFS;
use crawler::ehentai::EHentaiCrawler;
use crawler::nhentai::NHentaiCrawler;
use crawler::HxCrawler;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fs = Box::new(LocalFS::new("."));
    let helper = Box::new(CrawlerHelper::new(fs));
    let ehentai_crawler = EHentaiCrawler::new();
    let nhentai_crawler = NHentaiCrawler::new();
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 1 {
        println!("Usage: hxdown <url> [... <url>]");
        std::process::exit(-1);
    }
    for url in &argv[1..] {
        if url.matches(&"e-hentai.org").next().is_some() {
            ehentai_crawler.download(&helper, url).await?;
        }else if url.matches(&"nhentai.net").next().is_some() {
            nhentai_crawler.download(&helper, url).await?;
        }else{
            println!("Unknown website: {}", url);
        }
    }
    Ok(())
}