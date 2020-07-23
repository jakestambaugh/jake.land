#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> &'static str {
    "Hello, world! Welcome to Jake's blog"
}

#[get("/<post_id>")]
fn show_post(post_id: u64) -> String {
    format!("Post id: {}", post_id)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/post/", routes![show_post])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_post_31() {
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
