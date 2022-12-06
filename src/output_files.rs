use std::fs::{self, ReadDir};

struct FileControl {
    folder_name: &'static str,
    file_name: &'static str,
    overwrite: bool,
    is_file_exist: bool
}

impl FileControl {
    fn new(folder_name: &'static str, file_name: &'static str, overwrite: bool) -> Self {
        FileControl { folder_name, file_name, overwrite, is_file_exist: false }
    }
    fn folder_detail(&self) -> Option<ReadDir> {
        let read_folder = fs::read_dir(self.folder_name);
        if let Ok(read_dir) = read_folder {
            Some(read_dir)
        } else {
            None
        }
    }
}

pub fn write_html(files: String, path: &str) -> String {
    let result = fs::write(path, files);

    if let Ok(_) = result {
        String::from("Successfully Wrote the File")
    } else {
        String::from("Error on writing Files")
    }
}
