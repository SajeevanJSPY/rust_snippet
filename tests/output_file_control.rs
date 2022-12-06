#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]
mod tests {
    use std::fs;
    use std::io::{self, Read};
    use std::process;

    // Fields -> Folder Name, File Name, Overwrite
    #[test]
    fn check_folder() {
        let folder_name = "outputa";
        let file_name = "index.html";
        let dummy_content = "Hello World";
        let overwrite: bool = false;

        let read_folder = fs::read_dir(folder_name);
        
        let mut is_file_exist = false;

        if let Ok(files) = read_folder {
            for file in files {
                if let Ok(dir_entry) = file {
                    if dir_entry.file_name() == file_name {
                        is_file_exist = true;
                        break;
                    }
                }
            }
        } else {
            let create_folder = fs::create_dir(folder_name);
            if let Ok(_) = create_folder {
                let write_result = fs::write(format!("{folder_name}/{file_name}"), dummy_content);
                if let Err(err) = write_result {
                    println!("{}", err);
                }
            } else {
                println!("Error On Creating the Folder, try again...");
            }
        }

        // // If the file already exist in the folder, Ask Permission to overwrite it
        if is_file_exist && overwrite {
            print!("Do You want to overwrite {folder_name}/{file_name}: ");
            let mut permission_status = String::new();
            let permission_result = io::stdin().read_to_string(&mut permission_status);
            if let Ok(_) = permission_result {
                permission_status = permission_status.to_lowercase();
                if permission_status == "y" && permission_status == "yes" {
                    println!("Overwriting the File");                    
                }
            } else {
                process::exit(0);
            }
        }

    }
}
