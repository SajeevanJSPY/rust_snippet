use std::fs;

pub fn write_html(files: String, path: &str) -> String{
    let result = fs::write(path, files);

    if let Ok(_) = result {
        String::from("Successfully Wrote the File")
    } else {
        String::from("Error on writing Files")
    }
}