use std::fs;

pub fn write_the_html_file(files: String, path: &str) -> String{
    let result = fs::write(path, files);
    match result {
        Err(_e) => String::from("Error on writing Files"),
        Ok(_res) => String::from("Successfully Wrote the File")
    }
}