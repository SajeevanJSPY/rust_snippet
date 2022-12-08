use std::fs::{self, File};
use std::path::{Path, MAIN_SEPARATOR};

use crate::get_input;

pub struct FileControl {
    pub folder_name: String,
    pub file_name: String,
    pub overwrite: bool,
    pub is_file_exist: bool,
}

impl FileControl {
    pub fn new(folder_name: String, file_name: String, overwrite: bool) -> Self {
        FileControl {
            folder_name,
            file_name,
            overwrite,
            is_file_exist: false,
        }
    }

    fn create_file(&self) {
        let file_path = format!("{}{}{}", self.folder_name, MAIN_SEPARATOR, self.file_name);

        if let Ok(_) = File::create(file_path) {
            println!("File was created Successfully")
        }
    }

    fn set_file_name(&mut self) {
        self.file_name = get_input(String::from("Change your file_name..."));
    }

    pub fn check_file_path(&mut self) {
        let read_folder = Path::new(&self.folder_name);

        if read_folder.exists() {
            if let Ok(files) = read_folder.read_dir() {
                for file in files {
                    if let Ok(file_name) = file {
                        if let Ok(file_path) = file_name.file_name().into_string() {
                            if file_path == self.file_name {
                                self.is_file_exist = true;
                            }
                        }
                    }
                }
            }

            if !self.is_file_exist {
                self.create_file();
            }

            if self.is_file_exist && self.overwrite {
                let permission = get_input(String::from("you specified path already been taken, do you want to overwrite the file?(y/N) or change the file_name(rename|R): ")).to_lowercase();
                if permission == "y" || permission == "yes" {
                    self.create_file();
                } else if permission == "rename" || permission == "r" {
                    self.set_file_name();
                } else {
                    println!("Permission Failed!");
                }
            }
        } else {
            // Create the Folder
            let folder = fs::create_dir(&self.folder_name);
            if let Ok(_) = folder {
                self.create_file();
            }
        }
    }
}
