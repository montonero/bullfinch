use reqwest::Url;
use scraper::{html::Select, Html, Selector};

use crate::error::BfError;

/// A wrapper over scraper::Html etc that selects only links
pub struct UrlScraper {
    url: Url,
    html: Html,
    selector: Selector,
}

impl UrlScraper {
    ///
    /// * `url` - url of the page that should be parsed. This is required for relative links.
    /// * `text` - data to be parsed.
    pub fn new(url: Url, text: &str) -> Result<Self, BfError> {
        Ok(Self {
            // Url::parse(url)?
            url: url,
            html: Html::parse_document(text),
            selector: Selector::parse("a").expect("Could not create scraper Selector."),
        })
    }

    pub fn into_iter<'a>(&'a self) -> UrlIter<'a, 'a> {
        UrlIter {
            url: &self.url,
            data: self.html.select(&self.selector),
        }
    }
}

pub struct UrlIter<'a, 'b> {
    url: &'a Url,
    data: Select<'a, 'b>,
}

impl<'a, 'b> Iterator for UrlIter<'a, 'b> {
    type Item = Url;

    fn next(&mut self) -> Option<Self::Item> {
        for element in &mut self.data {
            if let Some(url) = element.value().attr("href") {
                if !url.starts_with("?") {
                    // Join links so that relative ones will be valid later
                    if let Ok(url) = self.url.join(url) {
                        return Some(url);
                    }
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::urlscraper::UrlScraper;
    use reqwest::Url;

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn urlscraper_parse_link_single() {
        let text = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <a href="mypage">link text</a>
        "#;

        let html = scraper::Html::parse_document(&text);
        let selector = scraper::Selector::parse("a").expect("failed to create selector");
        let mut data = html.select(&selector);
        let mut links = Vec::new();
        for element in &mut data {
            if let Some(url) = element.value().attr("href") {
                links.push(url)
            }
        }
        assert_eq!(1, links.len());
        assert_eq!("mypage", links[0]);
    }

    #[test]
    fn urlscraper_parse_link_single2() {
        let text = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <a href="mypage">link text</a>
        <a href="mypage">link text 2</a>
        "#;

        // All links should be the same
        let url = Url::parse("http://mydomain.com").unwrap();
        let url_scraper = UrlScraper::new(url, &text).unwrap();
        for x in url_scraper.into_iter() {
            assert_eq!("http://mydomain.com/mypage", x.into_string());
        }
    }
}
