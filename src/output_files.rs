use std::fs::{self, ReadDir};

pub struct FileControl {
    pub folder_name: &'static str,
    pub file_name: &'static str,
    pub overwrite: bool,
    pub is_file_exist: bool,
}

impl FileControl {
    pub fn new(folder_name: &'static str, file_name: &'static str, overwrite: bool) -> Self {
        FileControl {
            folder_name,
            file_name,
            overwrite,
            is_file_exist: false,
        }
    }
    pub fn folder_detail(&self) -> Option<ReadDir> {
        let read_folder = fs::read_dir(self.folder_name);
        if let Ok(read_dir) = read_folder {
            Some(read_dir)
        } else {
            None
        }
    }
    pub fn check_file_exist(&mut self) {
        if let Some(read_dir) = self.folder_detail() {
            for file in read_dir {
                if let Ok(dir_entry) = file {
                    if dir_entry.file_name() == self.file_name {
                        self.is_file_exist = true;
                    }
                }
            }
        }
    }
    pub fn create_folder(&self) {
        let create_folder = fs::create_dir(self.folder_name);
        if let Ok(_) = create_folder {
            let write_result = fs::write(format!("{}/{}", self.folder_name, self.file_name), "");
            if let Err(err) = write_result {
                println!("{}", err);
            }
        } else {
            println!("Error On Creating the Folder, try again...");
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
