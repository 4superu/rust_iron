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
    //static_page_router
    router.get("/", routes::top_handler, "index");
    router.post("/greet", routes::greet_handler, "greeting");
    //blog_router
    router.get("/blog", blog::blog_handler, "blog");
    router.get("/new_blog", blog::new_blog, "new_blog");
    router.get("/edit_blog", blog::edit_blog, "edit_blog");
    router.post("/register", blog::register, "register" );
    router.post("/update", blog::update, "update");

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
