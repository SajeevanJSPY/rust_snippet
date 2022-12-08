#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]

mod tests {
    use std::path::Path;
    use rust_snipper::output_files::FileControl;

    #[test]
    fn check_file_path() {
        let folder_name = String::from("output");
        let file_name = String::from("index.html");
        let mut file = FileControl::new(folder_name.clone(), file_name.clone(), true);
        file.check_file_path();

        // Explicitly Checking the File Exist Or Not
        let path = format!("{folder_name}/{file_name}");
        let file_path = Path::new(&path);
        assert_eq!(file_path.exists(), true);

        let folder_name2 = String::from("output2");
        let file_name2 = String::from("index2.html");
        let mut file2 = FileControl::new(folder_name2, file_name2, true);
        file2.check_file_path();

        // Explicitly Checking the File Exist Or Not
        let path2 = format!("{folder_name}/{file_name}");
        let file_path2 = Path::new(&path);
        assert_eq!(file_path2.exists(), true);
    }
}
