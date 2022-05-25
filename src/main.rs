#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::{serde::Serialize, time::Date, response::content};
use std::fs;

#[derive(Serialize)]
struct Link<'a> {
    title: &'a str, 
    url: &'a str,
}

struct Post<'a> {
    title: &'a str,
    date: &'a str,
    content: &'a str,
    links: Vec<Link<'a>>,
    tags: &'a str,
}

#[get("/style.css")]
fn get_css() -> content::RawCss<String> {
    let contents = fs::read_to_string("static/style.css").unwrap();

    content::RawCss(contents)
}

#[get("/script.js")]
fn get_javascript() -> content::RawJavaScript<String> {
    let contents = fs::read_to_string("static/script.js").unwrap();

    content::RawJavaScript(contents)
}

#[get("/posts")]
fn get_posts() -> Template {
    let link1 = Link {
        title: "Let's build a Database in C",
        url: "https://cstack.github.io/db_tutorial/"
    };

    let link2 = Link {
        title: "Pelado Nerd",
        url: "https://www.twitch.tv/rwxrob",
    };

    let link3 = Link {
        title: "rwxrob",
        url: "https://www.twitch.tv/rwxrob",
    };

    let links = vec![link1, link2, link3];

    let post = Post {
        title: "Test",
        date: "Written on 22nd May, 2022",
        content: "this is a test",
        links,
        tags: "#devops #test #hopethisworks"
    };

    Template::render("post", context! {
        title: post.title,
        date: post.date,
        content: post.content,
        links: post.links,
        tags: post.tags,
    })
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("this is a {}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("this is a {}", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("this is a {}", id)
}

#[get("/hola/<name>")]
fn hola(name: &str) -> String {
    format!("holis {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_posts])
        .mount("/", routes![get_css])
        .mount("/", routes![get_javascript])
        .mount("/", routes![hola])
        .mount("/", routes![user, user_int, user_str])
        .attach(Template::fairing())
}
