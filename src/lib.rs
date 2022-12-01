use std::{fs, string};

pub fn format_a_single_element(html_element: HTMLElement) -> String {
    format!("<{:?}>{}</{:?}>", html_element.name, html_element.content, html_element.name)
}

#[derive(Debug)]
pub enum HTMLKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

pub struct HTMLElement {
    pub name: HTMLKind,
    pub content: String
}

pub fn creating_the_proper_htmlfile(html_content: String) -> String {
    format!(
"<!DOCTYPE html>
<html lang=en'>
<head>
    <meta charset='UTF-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>{}</title>
</head>
<body>
    {}
</body>
</html>
",
    "Hello World",
    html_content
    )
}

pub fn write_the_html_file(files: String, path: &str) -> String{
    let result = fs::write(path, files);
    match result {
        Err(_e) => String::from("Error on writing Files"),
        Ok(_res) => String::from("Successfully Wrote the File")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}