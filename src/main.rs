#[macro_use] extern crate rocket;
use figment::Figment;

#[get("/hello?<name>")]
fn hello(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {name}!"),
        None => format!("Hello, world!")
    }
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::debug_default())
        .merge(("port", 80))
        .merge(("address", "192.168.212.37"));

    rocket::custom(figment)
        .mount("/", routes![hello])
}
