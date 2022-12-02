use std::fs;

pub fn format_a_single_element(html_element: HTMLElement) -> String {
    let single_tag = format!("{:?}", html_element.name).to_lowercase();
    format!("<{}>{}</{}>", single_tag, html_element.content, single_tag)
}

pub fn setting_up_formatter(html_element: HTMLElement) {
    let mut html_elements: Vec<HTMLElement> = vec![];

    match html_element.children {
        None => {
            ()
        }
        Some(_) => {
            ()
        }
    }
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
    pub content: String,
    pub children: Option<Vec<HTMLElement>>
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
        // Should Properly Structuring a single tag
        let h1 = HTMLElement {
            name: HTMLKind::H1,
            content: String::from("Hello World"),
            children: None
        };
        assert_eq!("H1", format!("{:?}", h1.name));

        let formatted_html_kind = format!("{:?}", h1.name);
        assert_eq!(formatted_html_kind.to_lowercase(), "h1");

        let formatted_and_lowercased_html_kind = formatted_html_kind.to_lowercase();
        let html_tag = format!("<{}>{}</{}>", formatted_and_lowercased_html_kind, h1.content, formatted_and_lowercased_html_kind);
        assert_eq!("<h1>Hello World</h1>", html_tag);
    }
}