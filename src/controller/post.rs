use crate::Link;
use rocket::serde::Serialize;
use pulldown_cmark::{Parser, html};

#[derive(Serialize)]
pub struct Post<'a> {
    title: &'a str,
    date: &'a str,
    content: String,
    links: Vec<Link<'a>>,
    tags: &'a str,
}

impl<'a> Post<'a> {
    pub fn new(title: &'a str, date: &'a str, content: &'a str, links: Vec<Link<'a>>, tags: &'a str) -> Self {
        let content = String::from(content);
        let html = Self::convert_to_html(&content);

        Self {
            title,
            date,
            content: html,
            links,
            tags
        }
    }

    fn convert_to_html(markdown: &str) -> String {
        let parser = Parser::new(&markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
