use rust_snipper::{HTMLElement, HTMLKind, format_a_single_element, write_the_html_file, creating_the_proper_htmlfile};

fn main() {
    let path = "output/index.html";

    let h1 = HTMLElement {
        name: HTMLKind::H1,
        content: String::from("Hello World")
    };

    let code = creating_the_proper_htmlfile(format_a_single_element(h1));
    write_the_html_file(code, path);
}
