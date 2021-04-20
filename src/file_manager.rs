use copy_dir::copy_dir;
use std::fs;


pub fn optionstr_2_string(text: Option<&str>) -> String {
    return String::from(text.unwrap());
}

pub struct FileManager;

impl FileManager {
    pub fn compare(file1: String, file2: String) -> String {
        let _content1 = FileManager::read_file(file1);
        let _content2 = FileManager::read_file(file2);
        let _result = "";

        // TODO: finish this function

        return "".to_string();
    }

    pub fn read_file(file_name: String) -> String {
        return fs::read_to_string(file_name).expect("Couldn't read file.");
    }

    pub fn clone(path1: String, path2: String) {
        // TODO: check if this copies files

        copy_dir(path1, path2).expect("Couldn't copy directory.");
    }

    // TODO: create function for finding out if path is directory or file

    pub fn is_dir(path: String) -> bool {
        let metadata = fs::metadata(&path).unwrap();

        return metadata.is_dir();
    }

    pub fn remove(path: String) {
        // TODO: fix this

        if FileManager::is_dir(path) {
            fs::remove_dir(path.clone()).expect("Couldn't remove directory.");
        }
        else {
            fs::remove_file(path.clone()).expect("Couldn't remove file.");
        }
    }

    pub fn touch(path: String) {
        if path.contains(".") {
            fs::File::create(path).expect("Couldn't create file.");
        }
        else {
            fs::create_dir(path).expect("Couldn't create directory.");
        }
    }
}
