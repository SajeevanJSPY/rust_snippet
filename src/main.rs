// Modules
mod output_files;
mod formatter;

// Getting into the scope
use formatter::{creating_the_proper_htmlfile, format_a_single_element};
use rust_snipper::{HTMLElement, HTMLKind};
use output_files::write_the_html_file;

fn main() {
    let path = "output/index.html";

    let h1 = HTMLElement {
        name: HTMLKind::H1,
        content: String::from("Hello World"),
        children: None
    };

    let code = creating_the_proper_htmlfile(format_a_single_element(h1.name, h1.content));
    write_the_html_file(code, path);
}
