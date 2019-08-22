//#[macro_use]
//extern crate json;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize};
//use failure::Fail;
//use json::JsonValue;
use std::collections::HashMap;
use std::sync::{Mutex};


use bullfinch::errors::BfError;
use bullfinch::Crawler;

#[derive(Deserialize, Debug)]
enum CrawlRequest {
    NumLinks,
    Links,
}

/*
#[derive(Deserialize, Debug)]
struct CrawlRequestData {
    //#[serde(rename = "domain")]
    domain: String,
    action: CrawlRequest,
}
*/

struct AppState {
    // Maps crawlers to their IDs
    flock: Mutex<HashMap<u32, Crawler>>,
    // Cache of the pairs (id, domain name)
    flock_domains: Mutex<HashMap<u32, String>>,
}

fn get_domains(state: web::Data<AppState>) -> Result<HttpResponse, BfError> {
    let flock_domains = &*state.flock_domains.lock().unwrap();
    //println!("domains: {:?}", flock_domains);
    Ok(HttpResponse::Ok().json(flock_domains)) 
}

fn get_num_visited(domain_id: web::Path<u32>, state: web::Data<AppState>) -> Result<HttpResponse, BfError> {
    let flock = state.flock.lock().unwrap();
    match flock.get(&*domain_id) {
        Some(domain) => {
            let visited = &*domain.visited.lock().unwrap();
            Ok(HttpResponse::Ok().json(visited.len()))
        },
        None => Err(BfError::DomainNotRegistered(*domain_id))
    }
}


fn get_visited(domain_id: web::Path<u32>, state: web::Data<AppState>) -> Result<HttpResponse, BfError> {
    let flock = state.flock.lock().unwrap();
    match flock.get(&*domain_id) {
        Some(domain) => {
            let visited = &*domain.visited.lock().unwrap();
            // Create a set of visited domains
            // TODO could cache this 
            let vis_str: std::collections::HashSet<String> = visited.iter().map(|url| url.to_string()).collect();
            Ok(HttpResponse::Ok().json(vis_str))
        }
        None => Err(BfError::DomainNotRegistered(*domain_id))
    }

}


fn main() -> Result<(), BfError> {
    println!("Server starting.");

    let domains = Mutex::new(HashMap::new());
    // Hard code two domains to crawl
    domains
        .lock()
        .unwrap()
        .insert(1, Crawler::new("http://google.com")?);
    domains
        .lock()
        .unwrap()
        .insert(2, Crawler::new("https://www.dailymail.co.uk/home/index.html")?);

    // Start crawling for all registered domains
    for domain in domains.lock().unwrap().values_mut() {
        domain.start()
    }

    let domains_str = Mutex::new(HashMap::new());
    for (id, crawler) in &*domains.lock().unwrap() {
        domains_str.lock().unwrap().insert(*id, crawler.domain.clone());
    }

    // Shared Actix App state
    let app_data = web::Data::new(AppState { flock: domains, flock_domains: domains_str });

    HttpServer::new(move || {
        App::new()
            .register_data(app_data.clone())
            .route("/api/v1/links/{domain_id}", web::get().to(get_visited))
            .route("/api/v1/num/{domain_id}", web::get().to(get_num_visited))
            .route("/api/v1/domains", web::get().to(get_domains))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();

    Ok(())
}
