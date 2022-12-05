use crate::{HTMLElement, HTMLKind};

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

pub fn single_tag(single_tag: String, tag_content: String) -> String {
    format!("<{single_tag}>{tag_content}</{single_tag}>")
}

pub fn _setting_up_formatter(html_element: HTMLElement) {
    // let mut html_elements: Vec<HTMLElement> = vec![];

    match html_element.children {
        None => {
            ()
        }
        Some(_) => {
            ()
        }
    }

}