#[macro_use] extern crate rocket;

#[get("/")]
fn home() -> &'static str {
    "Hello, world! Welcome to Jake's blog"
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![home])
}
