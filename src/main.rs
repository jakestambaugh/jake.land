#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::{StaticFiles, crate_relative};

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
            title: "First post",
            body: "A wonderful post for the new blog. This is a large piece of text that will hopefully support markdown in the future",
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
        .mount("/img/", StaticFiles::from(crate_relative!("static/images/")))
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
    }
}
