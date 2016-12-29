#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    

    server.get("/", middleware!("<h1>Nyanpassu!</h1>"));
    server.listen("127.0.0.1:7000");
}
