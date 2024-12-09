use std::fs;

#[allow(dead_code)]
pub fn get_input(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(contents) => return contents,
        Err(e) => panic!("Error reading file: {}", e),
    }
}
