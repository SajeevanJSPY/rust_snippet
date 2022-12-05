// Modules
pub mod formatter;
pub mod output_files;

#[derive(Debug, Clone)]
pub enum HTMLKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

#[derive(Debug, Clone)]
pub struct HTMLElement {
    name: HTMLKind,
    content: String,
    children: Option<Vec<HTMLElement>>
}

impl HTMLElement {
    fn new(name: HTMLKind, content: String, children: Option<Vec<Self>>) -> Self {
        HTMLElement { name, content, children }
    }
}