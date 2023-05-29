#[macro_use] extern crate rocket;

use rocket::fs::{NamedFile, FileServer};
use std::path::Path;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("web/").join("index.html")).await.ok()
}

#[get("/hello?<name>")]
fn hello(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {name}!"),
        None => format!("Hello, world!")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, index])
        .mount("/static", FileServer::from("web/static/"))
}
