use rust_snipper::formatter::html_file;
use rust_snipper::output_files::write_html;
use rust_snipper::{HTMLElement, HTMLKind};

fn main() {
    let path = "output/index.html";

    let h1 = HTMLElement::new(HTMLKind::H1, String::from("Hello World"), None);

    let code = html_file(h1.single_tag());
    write_html(code, path);
}
