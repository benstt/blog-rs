use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Link<'a> {
    title: &'a str,
    url: &'a str,
}

impl<'a> Link<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        Self {
            title,
            url
        }
    }
}
