use crate::Link;
use rocket::serde::Serialize;
use pulldown_cmark::{Parser, Event::{Start, Text}, Tag::{Heading, Paragraph, Emphasis, List}, HeadingLevel::H1, CowStr::Borrowed};
use regex::Regex;
use std::ops::Range;

#[derive(Serialize)]
pub struct Post<'a> {
    title: String,
    date: String,
    content: String,
    links: Vec<Link<'a>>,
    tags: String,
}

impl<'a> Post<'a> {
    pub fn new(title: &'a str, date: &'a str, content: &'a str, links: Vec<Link<'a>>, tags: &'a str) -> Self {
        // The content must be converted into HTML, as it may have
        // any tags whithin it.
        //
        // For example, if we have the body of the .md file like this:
        // *this* is ~~well-formatted~~ **badly formatted**
        // Then it'll simply render as is.
        // If we instead convert that to HTML then the .tera file
        // will understand the formatting and show the correct output.
        let content = String::from(content);
        let html = Self::convert_to_html(&content);

        Self {
            title: title.to_string(),
            date: date.to_string(),
            content: html,
            links,
            tags: tags.to_string()
        }
    }

    // pub fn new(markdown: &str) -> Self {

    // }

    /// Generates a Post from a markdown string.
    /// Takes the entire source text as input and splits every tag
    /// into its own field to the struct.
    pub fn from_markdown(markdown: &'a str) -> Self {
        let parser = Parser::new(&markdown);

        let mut heading = String::new();
        let mut date = String::new();
        let mut contents = String::new();
        let mut links: Vec<Link> = Vec::new();
        let mut tags = String::new();

        let link_regex: Regex = Regex::new(r#"^.*: <https?://..*>"#).unwrap();
        for cap in link_regex.captures_iter(&markdown) {
            let string = cap.get(0).map_or("", |m| m.as_str());
            let link = Link::new("", string); 
            links.push(link);
        }

        // workaround, passing an empty vector to the heading
        // won't let me compile
        let _v = vec![""];
        for (e, r) in parser.into_offset_iter() {
            let text = &markdown[r.start .. r.end];

            match &e {
                Start(Heading(H1, None, _v)) if _v.is_empty() => {
                    let title = &text 
                        .split("# ")
                        .collect::<Vec<&str>>();

                    heading.push_str(&title[1]);
                },
                Start(Paragraph) => {
                    if text.starts_with("#") {
                        tags.push_str(&text);
                    } else {
                        contents.push_str(&text);
                    }
                },
                Start(Emphasis) => {
                    if text.starts_with("Written on") {
                        date.push_str(&text);
                    }
                },
                _ => (),
            }
        }

        Self {
            title: heading,
            date,
            content: contents,
            links,
            tags
        }
    }

    /// Converts markdown to HTML.
    /// Takes markdown and generates an HTML file from it.
    fn convert_to_html(markdown: &str) -> String {
        let parser = Parser::new(&markdown);

        // TODO: all of this stuff should be on from_markdown()
        let mut heading = String::new();
        let mut date = String::new();
        let mut contents = String::new();
        let mut links: Vec<Link> = Vec::new();
        let mut tags = String::new();

        let mut related_section_range: Range<usize> = Range { start: 0, end: 0 };
        // workaround, passing an empty vector to the heading
        // won't let me compile
        let _v = vec![""];
        for (e, r) in parser.into_offset_iter() {
            let text = &markdown[r.start .. r.end];

            match &e {
                Start(Heading(H1, None, _v)) if _v.is_empty() => {
                    let title = &text 
                        .split("# ")
                        .collect::<Vec<&str>>();

                    heading.push_str(&title[1]);
                },
                Start(Paragraph) => {
                    if text.starts_with("#") {
                        tags.push_str(&text);
                    } else {
                        contents.push_str(&text);
                    }
                },
                Start(Emphasis) => {
                    if text.starts_with("Written on") {
                        date.push_str(&text);
                    }
                },
                Start(List(None)) => related_section_range = r,
                _ => (),
            }
        }

        let related_sections = 
            &markdown[related_section_range.start .. related_section_range.end].split("\n");

        let link_regex: Regex = Regex::new(r#"^.*: <https?://..*>"#).unwrap();
        // TODO: check regex for every line in the related section
        // for link in related_sections {

        // }
        let caps = link_regex.captures("* Google: <https://google.com>").unwrap();
        let text = caps.get(0).map_or("", |m| m.as_str());
        // for cap in link_regex.captures_iter(&related_sections) {
        //     let string = cap.get(0).map_or("", |m| m.as_str());
        //     println!("string: {}", string);
        //     let link = Link::new("", string); 
        //     links.push(link);
        // }

        // e: Start(Heading(H1, None, []))
        // e: Text(Borrowed("hey, this is a header"))
        // e: End(Heading(H1, None, []))
        // e: Start(Paragraph)
        // e: Text(Borrowed("and this is a paragraph"))
        // e: End(Paragraph)

        // parser.for_each(|e| println!("e: {:?}", e));

        let mut html_output = String::new();
        // html::push_html(&mut html_output, parser);

        html_output
    }

}
