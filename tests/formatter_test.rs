#[cfg(test)]
mod tests {
    use rust_snipper::{HTMLElement, HTMLKind};

    #[test] 
    fn single_tag() {
        let h1 = HTMLElement::new(HTMLKind::H1, String::from("Hello"), None);
        assert_eq!(h1.single_tag(), "<h1>Hello</h1>")
    }

    #[test]
    fn is_nested() {
        let h2_1 = HTMLElement::new(HTMLKind::H2, String::from(""), None);
        let h2_2 = HTMLElement::new(HTMLKind::H2, String::from(""), None);

        let h1 = HTMLElement::new(HTMLKind::H1, String::from("Hello"), Some(vec![h2_1, h2_2]));
        assert_eq!(h1.is_nested(), true)
    }
}