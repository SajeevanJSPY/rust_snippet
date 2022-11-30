use std::io;
use std::fs;

pub fn format_a_single_element(html_element: HTMLElement) -> String {
    format!("<{:?}>{}</{:?}>", html_element.name, html_element.content, html_element.name)
}

#[derive(Debug)]
pub enum HTMLKind {
    h1,
    h2,
    h3,
    h4,
    h5,
    h6
}

pub struct HTMLElement {
    pub name: HTMLKind,
    pub content: String
}

pub fn write_the_html_file(files: String, path: &str) -> String{
    let result = fs::write(path, files);
    match result {
        Err(e) => String::from("Error on writing Files"),
        Ok(res) => String::from("Successfully Wrote the File")
    }
}
