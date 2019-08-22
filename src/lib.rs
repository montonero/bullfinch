extern crate crossbeam_channel;

use reqwest::header::*;
use reqwest::{Client, Url};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
//use std::sync::mpsc::channel;
use crossbeam_channel as channel;

pub mod errors;
pub use errors::BfError;

mod urlscraper;
use urlscraper::UrlScraper;

fn is_same_domain(domain: &str, url: &Url) -> bool {
    match url.domain() {
        Some(domain_) => domain_ == domain,
        None => false,
    }
}

/// Single domain crawler
#[derive(Debug)]
pub struct Crawler {
    pub domain: String,
    domain_url: Url,
    pub visited: Arc<Mutex<HashSet<Url>>>,
    //pub frontier: VecDeque<Url>,
    /// Number of fetching threads
    num_fetchers: usize,
    pub crawl_depth: usize,
    /// Log links as they are processed or not (currently uses println)
    pub verbose_log: bool,
}

/*
TODO serialize this and use in Crawler
#[derive(Debug)]
pub struct Store {
    pub visited: HashSet<Url>,
}
*/

impl Crawler {
    /// Create new crawler with empty data
    pub fn new(domain: &str) -> Result<Self, BfError> {
        // Parse string and extract domain part
        let url = Url::parse(domain)?;
        let domain_str = url.domain().unwrap();
        let d: String = String::from(domain_str);
        //let v = Store { visited: HashSet::new() };
        let v = HashSet::new();
        Ok(Self {
            domain: d,
            domain_url: url,
            visited: Arc::new(Mutex::new(v)),
            //frontier: f,
            num_fetchers: 2,
            crawl_depth: 2,
            verbose_log: true,
        })
    }

    pub fn start(&mut self) {
        // If we encounter a link then send it to a master thread
        // it will run in a single thread whilst multiple worker threads will be fetching them
        let (master_tx, master_rx) = channel::unbounded::<(usize, Url)>();

        // After it has been verified that this is a valid link and we have not seen it yet
        // then send to worker threads for fetching
        let (worker_tx, worker_rx) = channel::unbounded::<(usize, Url)>();

        let client_ = Arc::new(Client::new());

        // Start crawling from the root
        match master_tx.send((0, self.domain_url.clone())) {
            Err(err) => println!("Error sending to master: {}", err),
            _ => (),
        }

        // Flag to signal worker threads to shut down
        let shutdown = Arc::new(AtomicBool::new(false));
        // Keep track of idle workers. We shall shutdown only if all have completed.
        let num_workers_idle = Arc::new(AtomicUsize::new(0));
        let num_fetchers = self.num_fetchers;
        for _ in 0..num_fetchers {
            let worker_rx = worker_rx.clone();
            let master_tx = master_tx.clone();
            let client = Arc::clone(&client_);
            let domain = self.domain.clone();
            let shutdown = Arc::clone(&shutdown);
            let max_depth = self.crawl_depth;
            let num_workers = Arc::clone(&num_workers_idle);

            thread::spawn(move || {
                num_workers.fetch_add(1, Ordering::SeqCst);
                for (depth, url) in worker_rx {
                    num_workers.fetch_sub(1, Ordering::SeqCst);
                    if shutdown.load(Ordering::SeqCst) {
                        println!("Worker thread shutting down.");
                        break;
                    }
                    if depth >= max_depth {
                        num_workers.fetch_add(1, Ordering::SeqCst);
                        continue;
                    }

                    let head = match client.head(url.clone()).send() {
                        Ok(head) => head,
                        Err(err) => {
                            println!("Error in getting head of {} : {}", url.as_str(), err);
                            num_workers.fetch_add(1, Ordering::SeqCst);
                            continue;
                        }
                    };
                    let headers = head.headers();
                    if let Some(content_type) =
                        headers.get(CONTENT_TYPE).and_then(|c| c.to_str().ok())
                    {
                        if content_type.starts_with("text/html") {
                            // If this is a html page then get it ...
                            let mut resp = client.get(url.clone()).send().unwrap();
                            let text = resp.text().unwrap();
                            // .. and parse
                            let url_scraper = UrlScraper::new(url, &text).unwrap();
                            // send all the links to master for processing
                            for link in url_scraper
                                .into_iter()
                                .filter(|u| is_same_domain(&domain, u))
                            {
                                match master_tx.send((depth + 1, link)) {
                                    Err(err) => {
                                        println!("Error sending to master: {}", err);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }

                    num_workers.fetch_add(1, Ordering::SeqCst);
                }
            });
        }

        let visited = Arc::clone(&self.visited);
        let crawl_depth = self.crawl_depth;
        let domain_url = self.domain_url.clone();
        let num_workers = num_workers_idle.clone();
        thread::spawn(move || {
            while !shutdown.load(Ordering::SeqCst) {
                let (depth, url) = match master_rx.try_recv() {
                    Ok((depth, url)) => (depth, url),
                    Err(_err) => {
                        thread::sleep(Duration::from_millis(1_000));
                        //println!("Error receiving {}", err);
                        // If all channels are empty then there is no work to be done
                        if worker_tx.is_empty() && worker_rx.is_empty() 
                        && master_rx.is_empty() && num_workers.load(Ordering::SeqCst) == num_fetchers {
                            shutdown.store(true, Ordering::SeqCst);
                            println!("No more work to do. Shutting down.");
                        }
                        continue;
                    }
                };

                if !is_same_domain(&domain_url.domain().unwrap(), &url) {
                    println!(
                        "Different domain: {} {}",
                        url,
                        &domain_url.domain().unwrap()
                    );
                    continue;
                }

                // Remove '#' fragments from url
                // to make sure that we don't store page.hml and page.html#abc as separate entries
                let mut url_f = url.clone();
                url_f.set_fragment(None);
                if visited.lock().unwrap().contains(&url_f) {
                    //if self.verbose_log {
                    //println!("Already visited {}", url);
                    //}
                } else {
                    //if self.verbose_log {
                    //    println!("Visiting: {} {}", depth, url);
                    //}
                    visited.lock().unwrap().insert(url_f);

                    if depth >= crawl_depth {
                        continue;
                    }

                    match worker_tx.send((depth, url)) {
                        Err(err) => {
                            println!("Error sending to worker channel {}. Quitting.", err);
                            shutdown.store(true, Ordering::SeqCst);
                        }
                        _ => (),
                    };
                    // Intentionally slowing down
                    // Let's behave responsibly
                    thread::sleep(Duration::from_millis(250));
                }
            }
        });
    }
}
