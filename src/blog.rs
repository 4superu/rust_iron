extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate rusqlite;
extern crate time;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::{url_for};
use hbs::{Template};
use iron::modifiers::{Redirect};
use params::{Params, Value};
use rusqlite::Connection;
use self::time::Timespec;


#[derive(Debug)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub body: String,
    pub time_posted: Timespec,
    pub time_updated: Timespec
}

pub fn blog_handler(req: &mut Request) -> IronResult<Response> {

    let mut resp = Response::new();
    let mut data = HashMap::new();

    let conn = Connection::open("sqlite3.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, body ,time_posted, time_updated FROM blog ORDER BY time_posted DESC").unwrap();
    let blog_iter = stmt.query_map(&[], |row| {
        Blog {
            id: row.get(0),
            title: row.get(1),
            author: row.get(2),
            body: row.get(3),
            time_posted: row.get(4),
            time_updated: row.get(5)
        }
    }).unwrap();

    let mut blog_list: Vec<Blog> = Vec::new();
    for blog in blog_iter {
        blog_list.push(blog.unwrap());
    }

    data.insert(String::from("blog_path"),
                format!("{}", url_for(req, "blog", HashMap::new())));
    data.insert(Vec::from("blog"), blog_list);

    resp.set_mut(Template::new("blog", data)).set_mut(status::Ok);

    return Ok(resp);
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

        let sentence = Blog {
            id: 0,
            title: blog_title.to_string(),
            author: blog_author.to_string(),
            body: blog_body.to_string(),
            time_posted: time::get_time(),
            time_updated: time::get_time()
        };
        conn.execute("INSERT INTO blog (title, author, body, time_posted, time_updated)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                    &[&sentence.title, &sentence.author, &sentence.body, &sentence.time_posted, &sentence.time_updated]).unwrap();
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
        let blog_time_posted = match map.find(&["time_posted"]) {
            Some(&Value::String(ref time_posted))  => {
                time_posted
            },
            _ => {
                "fail"
            }
        };

        let sentence = Blog {
            id: 0,
            title: blog_title.to_string(),
            author: blog_author.to_string(),
            body: blog_body.to_string(),
            time_posted: time::get_time(),
            time_updated: time::get_time()
        };

        conn.execute("UPDATE blog SET (title, author, body, time_posted, time_updated = datetime('now', 'localtime'))
                        VALUES (?1, ?2, ?3, ?4)",
                        &[&sentence.title, &sentence.author, &sentence.body, &sentence.time_posted]).unwrap();
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
    // data.insert(String::from("blog"), blog_vector);

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

// conn.execute("CREATE TABLE blog (
//               id              INTEGER PRIMARY KEY,
//               title           TEXT NOT NULL,
//               author          TEXT NOT NULL,
//               body            TEXT NOT NULL,
//               time_posted     REAL NOT NULL,
//               time_updated    REAL NOT NULL
//               )", &[]).unwrap();
