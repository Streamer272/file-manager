use copy_dir::copy_dir;
use std::{fs, env};

use crate::file_difference::FileDifference;


pub fn option2string(text: Option<&str>) -> String {
    return String::from(text.unwrap());
}

pub fn option2bool(boolean: Option<bool>) -> bool {
    return bool::from(boolean.unwrap());
}

pub struct FileManager;

impl FileManager {
    fn path_exists(path: &String) -> bool {
        return fs::metadata(path).is_ok();
    }

    pub fn compare(file1: String, file2: String) -> Option<String> {
        if !FileManager::path_exists(&file1) {
            println!("First file doesnt exist!");
            return None;
        }
        if !FileManager::path_exists(&file2) {
            println!("Second file doesnt exist!");
            return None;
        }

        // TODO: finish this function

        let content1 = String::from(FileManager::read_file((&file1).to_string())
            .unwrap());
        let content2 = String::from(FileManager::read_file((&file2).to_string())
            .unwrap());
        let mut result = String::new();

        // let mut index1 = 0;
        // let mut index2 = 0;

        // TODO: this throws index error

        // {
        //     let content2: Vec<&str> = content2.split(" ").collect();
        //
        //     for word in content1.split("\n") {
        //         if word == content2[index1] {
        //             result.push_str(content2[index2]);
        //         }
        //
        //         // if content1.chars().collect()[index1] == content2.chars().collect()[index2] {
        //         //     result.push(content1.chars().collect()[index1]);
        //         // }
        //
        //         index1 += 1;
        //         //index2 += 1;
        //     }
        // }

        let mut differences: Vec<String> = Vec::new();
        let mut difference_indexes: Vec<usize> = Vec::new();

        return Option::from(result.to_string());
    }

    pub fn read_file(file_name: String) -> Option<String> {
        if !FileManager::path_exists(&file_name) {
            println!("File doesnt exist!");
            return None;
        }

        return Option::from(fs::read_to_string(file_name).expect(
            "Couldn't read file."));
    }

    pub fn clone(path1: String, path2: String) -> bool {
        if !FileManager::path_exists(&path1) {
            println!("Clone path doesnt exist!");
            return true;
        }
        if FileManager::path_exists(&path2) {
            println!("Cloned path already exist!");
            return true;
        }

        if option2bool(FileManager::is_dir(&path1)) {
            copy_dir(&path1, &path2).expect("Couldn't copy directory.");
        }
        else {
            fs::File::create(&path2).expect("Couldn't copy file.");
        }

        return false;
    }

    pub fn is_dir(path: &String) -> Option<bool> {
        if !FileManager::path_exists(&path) {
            println!("Path doesnt exist!");
            return None;
        }

        let metadata = fs::metadata(&path).unwrap();

        return Option::from(metadata.is_dir());
    }

    pub fn remove(path: String) -> bool {
        if !FileManager::path_exists(&path) {
            println!("Path doesnt exist!");
            return true;
        }

        if option2bool(FileManager::is_dir(&path)) {
            // remove_dir_all also removes files in that directory (not just directory)
            fs::remove_dir_all(&path).expect("Couldn't remove directory.");
        }
        else {
            fs::remove_file(&path).expect("Couldn't remove file.");
        }

        return false;
    }

    pub fn touch(path: String, is_file: bool) -> bool {
        if FileManager::path_exists(&path) {
            println!("Path already exist!");
            return true;
        }

        if is_file {
            fs::File::create(path).expect("Couldn't create file.");
        }
        else {
            fs::create_dir(path).expect("Couldn't create directory.");
        }

        return false;
    }

    pub fn rename(path1: String, path2: String) -> bool {
        if !FileManager::path_exists(&path1) {
            println!("Rename path doesnt exist!");
            return true;
        }
        if FileManager::path_exists(&path2) {
            println!("Renamed path already exist!");
            return true;
        }

        fs::rename(&path1, &path2).expect("Couldn't rename file or directory.");

        return false;
    }

    pub fn get_working_directory() -> String {
        let path = env::current_dir().expect("Couldn't get working directory.");
        return path.display().to_string();
    }
}
