extern crate copy_dir;
use copy_dir::copy_dir;
use std::fs;


pub fn optionstr_2_string(text: Option<&str>) -> String {
    return String::from(text.unwrap());
}

pub struct FileManager;

impl FileManager {
    pub fn compare(file1: String, file2: String) -> String {
        let content1 = FileManager::read_file(file1);
        let content2 = FileManager::read_file(file2);
        let mut result = "";

        return "".to_string();
    }

    pub fn read_file(file_name: String) -> String {
        return fs::read_to_string(file_name).expect("Couldn't read file!");
    }

    pub fn clone(path1: String, path2: String) {
        copy_dir(path1, path2);
    }
}
