#[macro_use] extern crate rocket;

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
        .mount("/", routes![hello])
}
