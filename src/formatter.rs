use rust_snipper::HTMLElement;

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

pub fn format_a_single_element(html_element: HTMLElement) -> String {
    let single_tag = format!("{:?}", html_element.name).to_lowercase();
    format!("<{}>{}</{}>", single_tag, html_element.content, single_tag)
}

pub fn setting_up_formatter(html_element: HTMLElement) {
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

#[cfg(test)]
mod tests {
    use rust_snipper::{HTMLKind, HTMLElement};
    use super::format_a_single_element;

    #[test]
    fn setting_up_formatter() {
        // Should Properly Destructure the Nested Tags

        // HTML Nested Elements
        let child_element_1 = HTMLElement {
            content: String::from("Home"),
            name: HTMLKind::H3,
            children: None
        };
        
        let child_element_2 = HTMLElement {
            content: String::from("About"),
            name: HTMLKind::H3,
            children: None
        };

        
        let child_element_3 = HTMLElement {
            content: String::from("Status"),
            name: HTMLKind::H3,
            children: None
        };

        let main_element = HTMLElement {
            content: String::from(""),
            name: HTMLKind::H2,
            children: Some(vec![child_element_1, child_element_2, child_element_3])
        };

        let mut html_elements: Vec<HTMLElement> = vec![];
        
        // Destructure the Simply Nested Main Element
        match main_element.children {
            None => {
                format_a_single_element(main_element);
            }
            Some(vec) => {
                html_elements.extend(vec);
                println!("{:?}", html_elements);
            }
        }

        for ele in html_elements {
            match ele.children {
                None => {
                    let text_content = &ele.content;
                    assert_eq!(format!("<h3>{text_content}</h3>"), format_a_single_element(ele));
                }
                Some(_) => {
                    
                }
            }
        }

    }

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
