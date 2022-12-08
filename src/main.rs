// use rust_snipper::formatter::html_file;
use rust_snipper::output_files::FileControl;
// use rust_snipper::{HTMLElement, HTMLKind};

fn main() {
    let mut file_control = FileControl::new(String::from("output"), String::from("index.html"), true);
    file_control.check_file_path();
}
