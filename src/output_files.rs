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
        self.file_name = get_input(String::from("Input your file_name..."));
    }

    fn set_file_exist(&mut self) {
        let read_folder = Path::new(&self.folder_name);

        if let Ok(files) = read_folder.read_dir() {
            for file in files {
                if let Ok(file_name) = file {
                    if let Ok(file_path) = file_name.file_name().into_string() {
                        if file_path == self.file_name {
                            self.is_file_exist = true;
                            return;
                        }
                    }
                }
            }
        }

        self.is_file_exist = false;
    }

    pub fn check_file_path(&mut self) {
        let show_perm_rename = String::from("you specified path already been taken, do you want to overwrite the file?(y/N) or change the file_name(rename|R): ");
        let show_rename =
            String::from("you specified path already been taken, change the file_name(rename|R): ");

        let read_folder = Path::new(&self.folder_name);

        if read_folder.exists() {
            self.set_file_exist();

            if !self.is_file_exist {
                self.create_file();
            }

            if self.is_file_exist && self.overwrite {
                let permission = get_input(show_perm_rename).to_lowercase();
                if permission == "y" || permission == "yes" {
                    self.create_file();
                } else if permission == "rename" || permission == "r" {
                    let mut warn_count = 0;
                    while self.is_file_exist {
                        if warn_count > 0 {
                            println!("{}: file_name already taken", self.file_name);
                        }
                        self.set_file_name();
                        self.set_file_exist();
                        warn_count += 1;
                    }
                    self.create_file();
                } else {
                    println!("Permission Failed!");
                }
            }

            if self.is_file_exist && !self.overwrite {
                let permission = get_input(show_rename).to_lowercase();
                if permission == "rename" || permission == "r" {
                    self.set_file_name();
                    self.create_file();
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
