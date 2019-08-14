<p align="center">
  <img src="info/logo.png">
</p>

# Bullfinch

Bullfinch is an extremely simple web crawler written in Rust in one evening for learning purposes.

## Use

## Example


## Architecture

Visited links are stored in a HashSet, although Bloom filter might be a better choice[^1].

## Improvements
Immediate issues that should be addressed:
* Logging - use slog instead of println
* Error handling - close to none at the moment. Need to define our own error type and wrap all other errors[^2].
* Persistency - serialize and save visited links to disk

[^1]: http://www.michaelnielsen.org/ddi/how-to-crawl-a-quarter-billion-webpages-in-40-hours/
[^2]: https://blog.burntsushi.net/rust-error-handling/
