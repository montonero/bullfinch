use bullfinch::error::BfError;
use bullfinch::Crawler;

fn main() -> Result<(), BfError> {
    //let u = "https://www.theguardian.com/uk?INTCMP=CE_UK";
    //let u = "https://www.google.com";
    let u = "https://www.thetimes.co.uk/";
    //let u = "https://www.dailymail.co.uk/home/index.html";

    let mut crawler = Crawler::new(u)?;
    crawler.crawl_depth = 1;
    crawler.verbose_log = true;
    crawler.start();
    println!(
        "Found {} unique links (at a depth level {}).",
        crawler.visited.len(),
        crawler.crawl_depth
    );
    Ok(())
}
