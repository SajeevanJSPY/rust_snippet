#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]

mod tests {
    use rust_snipper::output_files::FileControl;

    #[test]
    fn check_file_path() {
        let mut file = FileControl::new("output_a", "file_a", true);
        file.check_file_path();
    }
}
