use std::io;

// Modules
pub mod formatter;
pub mod output_files;

// Reusable Functions + Macros
pub fn get_input(show: String) -> String {
    println!("{show}");
    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Err(_) = stdin.read_line(&mut buffer) {
        println!("not a valid input, try again...")
    }

    buffer.replace("\n", "")
}

#[derive(Debug, Clone)]
pub enum HTMLKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct HTMLElement {
    pub name: HTMLKind,
    pub content: String,
    pub children: Option<Vec<HTMLElement>>,
}

impl HTMLElement {
    pub fn new(name: HTMLKind, content: String, children: Option<Vec<Self>>) -> Self {
        HTMLElement {
            name,
            content,
            children,
        }
    }
    pub fn is_nested(&self) -> bool {
        if let None = self.children {
            false
        } else {
            true
        }
    }
    pub fn tag(&self) -> String {
        format!("{:?}", self.name).to_lowercase()
    }
    pub fn single_tag(&self) -> String {
        format!("<{}>{}</{}>", self.tag(), self.content, self.tag())
    }
}
