use std::fs::{self, File};
use std::path::{Path, MAIN_SEPARATOR};

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
    pub fn check_file_path(&mut self) {
        let file_path = format!("{}{}{}", self.folder_name, MAIN_SEPARATOR, self.file_name);
        let read_folder = Path::new(self.folder_name);

        if read_folder.exists() {
            if let Ok(files) = read_folder.read_dir() {
                for file in files {
                    if let Ok(file_name) = file {
                        if file_name.file_name() == self.file_name {
                            self.is_file_exist = true;
                        }
                    }
                }
            }

            if self.is_file_exist && self.overwrite {
                println!("You already have the file, do you want to overwrite the file?(y/N)");
            }

        } else {
            // Create the Folder
            let folder = fs::create_dir(self.folder_name);
            if let Ok(_) = folder {
                if let Ok(_) = File::create(file_path) {
                    println!("File was created Successfully")
                }
            }
        }
    }
}
