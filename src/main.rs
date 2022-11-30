use rust_snipper::{HTMLElement, HTMLKind, format_a_single_element, write_the_html_file};

fn main() {
    let path = "output/index.html";
    let h1 = HTMLElement {
        name: HTMLKind::H1,
        content: String::from("Hello World")
    };
    let files = format_a_single_element(h1);
    let result = write_the_html_file(files, path);
    println!("Status : {}", result);
}
