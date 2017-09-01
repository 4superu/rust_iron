extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate rusqlite;

use std::error::Error;
use iron::prelude::*;
use router::{Router};
use hbs::{HandlebarsEngine, DirectorySource};

mod routes;
mod blog;


fn main() {

    //Create Router
    let mut router = Router::new();
    let top_handler = routes::top_handler;
    let greet_handler = routes::greet_handler;
    let blog_handler = blog::blog_handler;
    let blog_generate_handler = blog::blog_generate_handler;

    router.get("/", top_handler, "index");
    router.post("/greet", greet_handler, "greeting");
    router.get("/blog", blog_handler, "blog");
    router.post("/blog",blog_generate_handler, "bloger");

    //Create Chain
    let mut chain = Chain::new(router);

    // Add HandlerbarsEngine to middleware Chain
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(
        DirectorySource::new("./src/templates/", ".hbs")
    ));

    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);

    println!("[+] Listen on localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
