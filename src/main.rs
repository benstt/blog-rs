#[macro_use] extern crate rocket;
use self::controller::post::*;
use self::controller::link::*;
use rocket_dyn_templates::{Template, context};
use rocket::{serde::Serialize, response::content};
use std::fs;

pub mod controller;

// // TODO: put this into a separate file
// #[derive(Serialize, Debug)]
// pub struct Link<'a> {
//     title: &'a str, 
//     url: &'a str,
// }

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

// TODO: put this into a separate file
#[get("/posts")]
fn get_posts() -> Template {
    let link1 = Link::new("Let's build a Database in C", "https://cstack.github.io/db_tutorial/");
    let link2 = Link::new("Pelado Nerd", "https://www.twitch.tv/rwxrob");
    let link3 = Link::new("rwxrob", "https://www.twitch.tv/rwxrob");

    let links = vec![link1, link2, link3];

    let post = Post::new("test", "written today", "**this is a test**", links, "#devops #test");

    Template::render("post", context! { post })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_posts])
        .mount("/", routes![get_css])
        .mount("/", routes![get_javascript])
        .attach(Template::fairing())
}
