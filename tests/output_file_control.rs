#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]
mod tests {
    use std::fs;

    // Fields -> Folder Name, File Name, Overwrite
    #[test]
    fn check_folder() {
        let folder_name = "output";
        let file_name = "test.html";
        let overwrite: bool = true;

        let folder_exist = fs::read_dir(folder_name);
        if let Ok(files) = folder_exist {
            println!("Folder Exist");
        } else {
            println!("Failed");
        }
    }
}