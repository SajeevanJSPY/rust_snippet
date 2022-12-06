#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]

mod tests {
    use rust_snipper::output_files::FileControl;

    #[test]
    fn folder_detail() {
        let file_data = FileControl::new("outputs", "index.html", false);
    }
}
