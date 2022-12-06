#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]

mod tests {
    use rust_snipper::output_files::FileControl;

    #[test]
    fn check_file_exist() {
        let mut file_data = FileControl::new("outputs", "index.html", false);
        file_data.check_file_exist();
        assert_eq!(file_data.is_file_exist, false);

        let mut file_data = FileControl::new("output", "index.html", false);
        file_data.check_file_exist();
        assert_eq!(file_data.is_file_exist, true);
    }
}
