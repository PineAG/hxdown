mod fs;
mod crawler;
mod err;

use crawler::CrawlerHelper;
use fs::local::LocalFS;
use crawler::ehentai::EHentaiCrawler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fs = LocalFS::new("./downloads");
    let helper = CrawlerHelper::new(&fs);
    let crawler = EHentaiCrawler::new();
    let urls = vec![
        // "https://e-hentai.org/g/1867753/56f430dfe0/",
        // "https://e-hentai.org/g/1867754/d624eea5c0/",
        // "https://e-hentai.org/g/1770584/57a87dbcf0/", // too long
        // "https://e-hentai.org/g/1766322/e3842e3c5f/",
        // "https://e-hentai.org/g/1747737/32b1158815/",
        "https://e-hentai.org/g/1462803/96b08e79d6/",
        "https://e-hentai.org/g/1176168/1f29e50602/",
        "https://e-hentai.org/g/1955555/10febc299f/",
        "https://e-hentai.org/g/1689618/a28178d144/",
        "https://e-hentai.org/g/1957924/04e50d43f4/",
        "https://e-hentai.org/g/1932906/8fbec13dcb/",
        "https://e-hentai.org/g/1915571/42cf52edbb/",
        "https://e-hentai.org/g/954734/31afec636d/",
        //New
        "https://e-hentai.org/g/1957071/4212e1c603/",
        "https://e-hentai.org/g/1936358/2d818a558c/",
        "https://e-hentai.org/g/1912140/5a6447fb8b/",
        "https://e-hentai.org/g/1850257/7011ace351/",
        "https://e-hentai.org/g/1851584/f268b069ae/",
        "https://e-hentai.org/g/1822547/5b60885104/",
        "https://e-hentai.org/g/1802829/64cac7a3c3/",
        "https://e-hentai.org/g/1260317/202cc4a34f/",
        "https://e-hentai.org/g/1816262/b37d7a69cd/", // syounen hi-ro-
        // New New
        "https://e-hentai.org/g/1875893/8f0ba99bbd/",
        "https://e-hentai.org/g/1967264/2d8d4724d0/"


    ];
    for url in urls {
        crawler.download(&helper, url).await?;
    }
    Ok(())
}