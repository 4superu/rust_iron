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
use iron::modifiers::{Redirect};
use rusqlite::Connection;
use params::{Params, Value};

#[derive(Debug)]
struct Blog {
    id: u32,
    title: String,
    author: String,
    body: String
}

pub fn blog_handler(req: &mut Request) -> IronResult<Response> {

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

    let mut resp = Response::new();
    let mut data = HashMap::new();

    data.insert(String::from("blog_path"),
                format!("{}", url_for(req, "blog", HashMap::new())));

    resp.set_mut(Template::new("blog", data)).set_mut(status::Ok);

    let conn = Connection::open("sqlite3.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, body FROM blog").unwrap();

    return Ok(resp)
}

//blog登録,blog一覧へRedirect
pub fn register(req: &mut Request) -> IronResult<Response> {

    let conn = Connection::open("sqlite3.db").unwrap();
    {
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
    }
    let ref blog_url = url_for(req, "blog", HashMap::new());
    return Ok(Response::with((status::Found, Redirect(blog_url.clone()))))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    let conn = Connection::open("sqlite3.db").unwrap();
    {
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
        conn.execute("UPDATE blog SET (title, author, body)
                        VALUES (?1, ?2, ?3)",
                        &[&blog_title, &blog_author, &blog_body]).unwrap();
    }
    let ref blog_url = url_for(req, "blog", HashMap::new());
    return Ok(Response::with((status::Found, Redirect(blog_url.clone()))))
}

pub fn edit_blog(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = HashMap::new();

    data.insert(String::from("edit_blog_path"),
                format!("{}", url_for(req, "edit_blog", HashMap::new())));

    resp.set_mut(Template::new("edit_blog", data)).set_mut(status::Ok);

    return Ok(resp);
}

pub fn new_blog(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = HashMap::new();

    data.insert(String::from("new_blog_path"),
                format!("{}", url_for(req, "new_blog", HashMap::new())));

    resp.set_mut(Template::new("new_blog", data)).set_mut(status::Ok);

    return Ok(resp);
}


// pub fn table_create() {
//
// }
// 怪しいコードなので見送り
// let blog_title = &map.find(&["title"]).unwrap();
// let blog_author = &map.find(&["author"]).unwrap();
// let blog_body = &map.find(&["body"]).unwrap();
