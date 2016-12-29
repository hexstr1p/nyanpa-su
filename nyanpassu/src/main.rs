#[macro_use] extern crate nickel;
extern crate hyper;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;
use hyper::header::Location;

fn main() {
    let mut server = Nickel::new();
    let short_urls = Arc::new(Mutex::new(HashMap::new()));
    short_urls.lock().unwrap().insert("rust".to_string(), "https://rust-lang.org".to_string());

    server.get("/", middleware!("<h1>Nyanpassu!</h1>"));

    let short_urls_clone = short_urls.clone();
    server.get("/:shortkey", middleware! {|request, mut response|
        let short_urls = short_urls_clone.lock().unwrap();
        let shortkey = request.param("shortkey").unwrap();
        if short_urls.contains_key(shortkey) {
            let url = short_urls.get(shortkey).unwrap();
            response.set(StatusCode::TemporaryRedirect);
            response.set(Location(url.clone()));
            return response.send("");
        }
        else {
            return response.send("short url not found");
        }
    });
    server.listen("127.0.0.1:7000");
}
