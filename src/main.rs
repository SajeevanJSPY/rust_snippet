use rust_snipper::{HTMLElement, HTMLKind};
use rust_snipper::formatter::html_file;
use rust_snipper::output_files::write_html;

fn main() {
    let path = "output/index.html";

    let h1 = HTMLElement {
        name: HTMLKind::H1,
        content: String::from("Hello World"),
        children: None
    };

    let code = html_file(h1.single_tag());
    write_html(code, path);
}
