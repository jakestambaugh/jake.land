#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::{crate_relative, StaticFiles};
use rocket_contrib::templates::Template;
use std::collections::HashMap;

mod jakeland;

#[derive(serde::Serialize)]
struct BlogPost {
    title: &'static str,
    body: &'static str,
}

#[derive(serde::Serialize)]
struct HomeContext {
    title: String,
    blogposts: Vec<BlogPost>,
    // This key tells handlebars whigh template is the parent.
    parent: &'static str,
}

#[get("/")]
fn home() -> Template {
    let context = HomeContext {
        title: "Blog posts".to_string(),
        blogposts: vec![BlogPost {
            title: "Placeholder",
            body: "A wonderful placeholder post for the new blog. Some day there will be interesting information displayed here. Unfortunately, today is not that day.",
        }],
        parent: "layout",
    };
    Template::render("blog", &context)
}

#[get("/about")]
fn about() -> Template {
    let mut context: HashMap<&'static str, &'static str> = HashMap::new();
    context.insert("title", "About Me");
    context.insert("parent", "layout");
    Template::render("about", &context)
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
        .mount(
            "/img/",
            StaticFiles::from(crate_relative!("static/images/")),
        )
        .mount("/css", StaticFiles::from(crate_relative!("static/css/")))
        .mount("/", routes![home, about])
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
        assert_eq!(response.into_string(), Some("Post id: 31".into()))
    }

    #[test]
    fn test_home() {
        use rocket::local::blocking::Client;

        let client = Client::new(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
