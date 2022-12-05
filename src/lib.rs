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

impl HTMLKind {
    pub fn tag(html_kind: HTMLKind) -> String {
        format!("{:?}", html_kind).to_lowercase()
    }
}

#[derive(Debug, Clone)]
pub struct HTMLElement {
    name: String,
    content: String,
    children: Option<Vec<HTMLElement>>
}

impl HTMLElement {
    pub fn new(name: String, content: String, children: Option<Vec<Self>>) -> Self {
        HTMLElement { name, content, children }
    }
}