#[derive(Debug)]
pub enum HTMLKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

#[derive(Debug)]
pub struct HTMLElement {
    pub name: HTMLKind,
    pub content: String,
    pub children: Option<Vec<HTMLElement>>
}

#[cfg(test)]
mod tests {
    
}