use std::fs;

pub fn read_chip_file(file_name:&str) -> String {
    // first get the file name
    let complete_path = "../chips/".to_owned() + file_name;
    let file_contents = fs::read_to_string(&complete_path).expect(&format!("Error in reading {}", complete_path));
    file_contents
}