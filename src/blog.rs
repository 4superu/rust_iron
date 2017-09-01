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
use rusqlite::Connection;

#[derive(Debug)]
struct Blog {
    id: i32,
    title: String,
    author: String,
    body: String
}

pub fn blog_handler(req: &mut Request) -> IronResult<Response> {

    let mut resp = Response::new();
    let mut data = HashMap::new();

    data.insert(String::from("bloger_path"),
                format!("{}", url_for(req, "bloger", HashMap::new())));

    resp.set_mut(Template::new("blog", data)).set_mut(status::Ok);

    let conn = Connection::open("sqlite3.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, body FROM blog").unwrap();

    let blog_iter = stmt.query_map(&[], |row| {
        Blog {
            id: row.get(0),
            title: row.get(1),
            author: row.get(2),
            body: row.get(3)
        }
    }).unwrap();

    return Ok(resp)
}

pub fn blog_generate_handler(req: &mut Request) -> IronResult<Response> {

    let conn = Connection::open("sqlite3.db").unwrap();
    use params::{Params, Value};
    let map = req.get_ref::<Params>().unwrap();

    let blog_title = match map.find(&["title"]) {
        Some(&Value::String(ref title))  => {
            title
        },
        _ => {
            "fail"
        }
    };
    let blog_author = match map.find(&["author"]) {
        Some(&Value::String(ref author))  => {
            author
        },
        _ => {
            "fail"
        }
    };
    let blog_body = match map.find(&["body"]) {
        Some(&Value::String(ref body))  => {
            body
        },
        _ => {
            "fail"
        }
    };
    conn.execute("INSERT INTO blog (title, author, body)
                    VALUES (?1, ?2, ?3)",
                    &[&blog_title, &blog_author, &blog_body]).unwrap();

    let mut stmt = conn.prepare("SELECT id, title, author, body FROM blog").unwrap();

    let blog_iter = stmt.query_map(&[], |row| {
        Blog {
            id: row.get(0),
            title: row.get(1),
            author: row.get(2),
            body: row.get(3)
        }
    }).unwrap();

    return Ok(Response::with((status::Ok)))
}

// pub fn blog_table_create_handler {
//     let conn = Connection::open(sqlite3.db).unwrap();
//
//     conn.execute("CREATE TABLE blog (
//                   id              INTEGER PRIMARY KEY,
//                   title           TEXT NOT NULL,
//                   author          TEXT NOT NULL,
//                   body            TEXT NOT NULL
//                   )", &[]).unwrap();
// }
