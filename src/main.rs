#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

use rocket::response::Redirect;
use rocket_contrib::serve::{crate_relative, StaticFiles};
use rocket_contrib::templates::Template;
use std::collections::HashMap;

mod jakeland;
use jakeland::BlogPost;

lazy_static! {
    static ref BLOGPOSTS: Vec<BlogPost> = Vec::new();
}

#[derive(serde::Serialize)]
struct HomeContext {
    title: String,
    blogposts: &'static Vec<BlogPost>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn home() -> Template {
    let context = HomeContext {
        title: "Blog posts".to_string(),
        blogposts: BlogPost::get_all(),
        parent: "layout",
    };
    Template::render("home", &context)
}

#[get("/about")]
fn about() -> Template {
    let mut context: HashMap<&'static str, &'static str> = HashMap::new();
    context.insert("title", "About Me");
    context.insert("parent", "layout");
    Template::render("about", &context)
}

#[derive(serde::Serialize)]
struct PostContext<'a> {
    blogpost: &'a BlogPost,
    parent: &'static str,
}

#[get("/<post_id>")]
fn show_post(post_id: usize) -> Template {
    let context = PostContext {
        blogpost: &BLOGPOSTS[post_id],
        parent: "layout",
    };
    Template::render("fullpost", context)
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
        .mount("/post/", routes![show_post])
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
