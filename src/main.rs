use rust_snipper::{HTMLElement, HTMLKind};
use rust_snipper::formatter::creating_the_proper_htmlfile;
use rust_snipper::output_files::write_the_html_file;

fn main() {
    let path = "output/index.html";

    let h1 = HTMLElement {
        name: HTMLKind::H1,
        content: String::from("Hello World"),
        children: None
    };

    let code = creating_the_proper_htmlfile(h1.single_tag());
    write_the_html_file(code, path);
}
