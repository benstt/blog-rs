#[macro_use] extern crate rocket;
use self::controller::post::*;
use self::controller::link::*;
use rocket::fs::{FileServer, relative};
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template, context};
use std::fs;
use std::path::{PathBuf, Path};

pub mod controller;

#[get("/static/<path..>")]
async fn get_static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static/")).join(path);

    NamedFile::open(path).await.ok()
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
        .mount("/", routes![get_static_files])
        .mount("/", FileServer::from(relative!("/static")))
        .attach(Template::fairing())
}
