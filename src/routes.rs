extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate rusqlite;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::{url_for};
use hbs::{Template};

pub fn top_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = HashMap::new();

    data.insert(String::from("greeting_path"),
                format!("{}", url_for(req, "greeting", HashMap::new())));

    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn greet_handler(req: &mut Request) -> IronResult<Response> {

    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    return match map.find(&["name"]) {

        Some(&Value::String(ref name)) => {
            Ok(Response::with(
                (status::Ok,
                                     format!("Hello {}", name).as_str())
            ))
        },
        _ => Ok(Response::with((status::Ok, "Hello world")))
    }
}
