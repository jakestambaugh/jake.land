#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::templates::Template;

mod jakeland;

#[get("/")]
fn home() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}

#[get("/<post_id>")]
fn show_post(post_id: u64) -> String {
    format!("Post id: {}", post_id)
}

#[post("/")]
fn create_post() -> String {
    let post = jakeland::Post::new();
    format!("Post #{}: *{}* - by {}", post.id, post.author, post.body)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![home])
        .mount("/post/", routes![show_post, create_post])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_post_id() {
        use rocket::local::blocking::Client;

        let client = Client::new(rocket()).unwrap();
        let response = client.get("/post/31").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some("Post id: 31".into())
        )
    }

    #[test]
    fn test_home() {
        use rocket::local::blocking::Client;

        let client = Client::new(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some("Hello, world! Welcome to Jake's blog".into())
        )
    }
}
