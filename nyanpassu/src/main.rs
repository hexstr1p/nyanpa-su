#[macro_use] extern crate nickel;
extern crate hyper;
extern crate url;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sid::io::Read;
use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;
use hyper::header::Location;
use hyper::Url;
use url::form_urlencoded;

fn main() {
    let mut server = Nickel::new();
    let short_urls = Arc::new(Mutex::new(HashMap::new()));
    short_urls.lock().unwrap().insert("rust".to_string(), "https://rust-lang.org".to_string());

    let short_urls_clone = short_urls.clone();
    server.get("/", middleware!{|_, response|
        let mut data = HashMap::new();
        let short_urls = short_urls_clone.lock().unwrap();
        data.insert("url_count", short_urls.len().to_string());
        return response.render("templates/index.tpl", &data);
    });

    let short_urls_clone = short_urls.clone();
    server.post("/shorten", middleware!{|request, response|
        let mut data = HashMap::new();
        let short_urls = short_urls_clone.lock().unwrap();
        data.insert("url_count", short_urls.len().to_string());

        let mut post_data = String::new();
        request.origin.read_to_string(&mut post_data).unwrap();
        let form = parse_form(post_data.as_bytes());

        let url = form.get("url").unwrap_or(&"".to_string()).to_string();
        if url != "" {
            if Url::parse(&url).is_ok() {
                data.insert("result", "this is a good url".to_string());
            }
            else {
                data.insert("result", "not a good url".to_string());
            }
        }
        else {
            data.insert("result", "enter a url".to_string()):
        }
        return response.render("templates/index.tpl", &data);
    });

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

fn parse_form(form_data: &[u8]) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    let parsed_form = form_urlencoded::parse(form_data);
    for (key, value) in parsed_form {
        hashmap.insert(key, value);
    }
    return hashmap;
}
