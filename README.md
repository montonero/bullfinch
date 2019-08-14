<p align="center">
  <img src="info/logo.png">
</p>

# Bullfinch

Bullfinch is an extremely simple web crawler written in Rust for learning purposes.

## Use

```cargo run --bin bf-client -- -v links  http://www.google.com 1 ```

## Example

```
use bullfinch::error::BfError;
use bullfinch::Crawler;

fn main() -> Result<(), BfError> {
    let u = "https://www.thetimes.co.uk/";


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

```

## Architecture
### Current
Currently there is just a single binary that serves as a command line interface (CLI) and a scraper.
![](info/architecture_initial.png)

Visited links are stored in a HashSet, although Bloom filter might be a better choice[^1].
Main thread checks whether the link that we encounter has been visited, if not it is sent to worker threads. Communication is implemented using crossbeam-channel.
Other approuch would have been to use Arc<Mutex<>> queue to append new links for fetching.

### Planned
![](info/architecture_goal.png)

## Improvements
Immediate issues that should be addressed:
*
* Logging - use slog instead of println
* Error handling - close to none at the moment. Need to define our own error type and wrap all other errors[^2].
* Persistency - serialize and save visited links to disk

[^1]: http://www.michaelnielsen.org/ddi/how-to-crawl-a-quarter-billion-webpages-in-40-hours/
[^2]: https://blog.burntsushi.net/rust-error-handling/
