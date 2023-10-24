mod handlers;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Rustfit"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, handlers::account::post_user])
}
