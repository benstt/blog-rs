use crate::Link;
use rocket::serde::Serialize;
use pulldown_cmark::{html, Parser, Event::{Start, End}, Tag::{Heading, Paragraph, Emphasis, List}, HeadingLevel::H1};
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
    pub fn new(markdown: &'a str) -> Self {
        Self::from_markdown(&markdown)
    }

    /// Generates a Post from a markdown string.
    /// Takes the entire source text as input and splits every tag
    /// into its own field to the struct.
    pub fn from_markdown(markdown: &'a str) -> Self {
        let parser = Parser::new(&markdown);

        let mut title = String::new();
        let mut date = String::new();
        let mut content = String::new();
        let mut links: Vec<Link> = Vec::new();
        let mut tags = String::new();

        let mut related_section_range: Range<usize> = Range { start: 0, end: 0 };

        // Check every event of the parser and get its range of text
        // Example:
        // > Start(Paragraph), Range[4..54]
        // Means that we stumbled upon the start of a paragraph, and the text
        // is located at positions 4 to 54 in the str provided to the parser
        for (e, r) in parser.into_offset_iter() {
            let text = &markdown[r.start .. r.end];

            match &e {
                Start(Heading(H1, _, _)) => {
                    let the_title = &text 
                        .split("# ")
                        .collect::<Vec<&str>>();

                    title.push_str(&the_title[1]);
                },
                Start(Heading(_, _, _)) => {
                    content.push_str(&text);
                    content.push_str("\n\n");
                },
                Start(Paragraph) => {
                    // Don't append the date or 'related:' to the text
                    let date_text = "*Written on";
                    let related_text = "Related:";
                    if text.starts_with(date_text) || text.starts_with(related_text) { continue }

                    if text.starts_with("#") {
                        tags.push_str(&text);
                        continue;
                    } 

                    for line in text.to_string().lines() {
                        // I seriously don't know, but if I don't put
                        // '</br>\n' the thing doesn't get correctly formatted.
                        // So annoying.
                        content.push_str(&format!("{}</br>\n", line));
                    }
                },
                Start(Emphasis) => {
                    if text.starts_with("*Written on") {
                        let text_without_asterisks = &markdown[r.start+1 .. r.end-1];
                        date.push_str(&text_without_asterisks);
                    }
                },
                Start(List(_)) => {
                    if text.starts_with("*") {
                        related_section_range = r;
                    } else {
                        content.push_str(&text);
                    }
                },
                End(Paragraph) => content.push_str("\n"),
                _ => (),
            }
        }

        let related_sections = 
            &markdown[related_section_range.start .. related_section_range.end]
                .split("\n")
                .take_while(|l| !l.is_empty());

        let src_re: Regex = Regex::new(r#"\*\s(.*):\s"#).unwrap();
        let link_re: Regex = Regex::new(r#"https?://..*"#).unwrap();

        // Check every line to match a title and a URL
        for link in related_sections.clone().into_iter() {
            // Look for matches with the regex
            let src_cap = src_re.captures(link).expect("Post has no references");
            let link_cap = link_re.captures(link).expect("Post has no links");
            
            // Here get(1) corresponds to the capture without the '*' and ':'
            // For example, capturing "* My site: " would return:
            // Some(Captures({
            //     0: Some("* My site: "),
            //     1: Some("My site"), <- this is the capture that we want
            // })),
            let title = src_cap.get(1).map_or("", |m| m.as_str());
            // Remove the first and last two asterisks of the title
            let title = &title[2..title.len()-2];
            let url = link_cap.get(0).map_or("", |m| m.as_str());

            links.push(Link::new(title, url));
        }
        
        // The content must be converted into HTML, as it may have
        // any tags whithin it.
        //
        // For example, if we have the body of the .md file like this:
        // *this* is ~~well-formatted~~ **badly formatted**
        // Then it'll simply render as is.
        // If we instead convert that to HTML then the .tera file
        // will understand the formatting and show the correct output.
        let content = Self::convert_to_html(&content);

        Self {
            title,
            date,
            content,
            links,
            tags
        }
    }

    /// Converts markdown to HTML.
    /// Takes markdown and generates an HTML file from it.
    fn convert_to_html(markdown: &str) -> String {
        let parser = Parser::new(&markdown);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

}
