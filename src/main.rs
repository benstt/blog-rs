#[macro_use] extern crate rocket;
use self::controller::post::*;
use self::controller::link::*;
use rocket_dyn_templates::{Template, context};
use rocket::response::content;
use std::fs;

pub mod controller;

#[get("/style.css")]
fn get_css() -> content::RawCss<String> {
    let contents = fs::read_to_string("static/style.css").unwrap();

    content::RawCss(contents)
}

#[get("/code-colorscheme.min.css")]
fn get_colorscheme() -> content::RawCss<String> {
    let contents = fs::read_to_string("static/code-colorscheme.min.css").unwrap();

    content::RawCss(contents)
}

#[get("/script.js")]
fn get_javascript() -> content::RawJavaScript<String> {
    let contents = fs::read_to_string("static/script.js").unwrap();

    content::RawJavaScript(contents)
}

// TODO: put this into a separate file
#[get("/posts")]
fn get_posts() -> Template {
    let content_file = fs::read_to_string("static/entries/02_building_a_backend.md")
        .expect("No such file or directory");

    let post = Post::new(&content_file);
    Template::render("post", context! { post })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_posts])
        .mount("/", routes![get_css])
        .mount("/", routes![get_colorscheme])
        .mount("/", routes![get_javascript])
        .attach(Template::fairing())
}
