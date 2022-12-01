use std::fs;

pub fn format_a_single_element(html_element: HTMLElement) -> String {
    let html_tag = format!("{:?}", html_element.name).to_lowercase().replace("\"", "");
    format!("<{:?}>{}</{:?}>", html_tag, html_element.content, html_tag)
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
<html lang='en'>
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
    ).replace("\'", "\"")
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
    use crate::{HTMLElement, HTMLKind};

    #[test]
    fn formatted_element(){ 
    //     let html_tag = format!("{:?}", html_element.name).to_lowercase().replace("\"", "");
    // format!("<{:?}>{}</{:?}>", html_tag, html_element.content, html_tag)

        let h1 = HTMLElement {
            name: HTMLKind::H1,
            content: String::from("Hello World")
        };
        assert_eq!("H1", format!("{:?}", h1.name));

        let formatted_html_kind = format!("{:?}", h1.name);
        assert_eq!(formatted_html_kind.to_lowercase(), "h1");

        let formatted_and_lowercased_html_kind = formatted_html_kind.to_lowercase();
        let html_tag = format!("<{}>{}</{}>", formatted_and_lowercased_html_kind, h1.content, formatted_and_lowercased_html_kind);
        assert_eq!("<h1>Hello World</h1>", html_tag);
    }
}