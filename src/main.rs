#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hugo")]
fn hugo() -> &'static str {
    "Hello, Hugo!"
}

#[get("/<name>")]
fn generic(name: &str) -> String {
    format!("Hello, {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hugo, generic])
}
