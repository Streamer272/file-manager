use copy_dir::copy_dir;
use std::{fs, env};


pub fn option2string(text: Option<&str>) -> String {
    return String::from(text.unwrap());
}

pub fn option2bool(boolean: Option<bool>) -> bool {
    return bool::from(boolean.unwrap());
}

pub struct FileManager;

impl FileManager {
    fn path_exists(path: String) -> bool {
        return fs::metadata(path).is_ok();
    }

    pub fn compare(file1: String, file2: String) -> Option<String> {

        if !FileManager::path_exists(file1.clone()) {
            println!("First file doesnt exist!");
            return None;
        }
        if !FileManager::path_exists(file2.clone()) {
            println!("Second file doesnt exist!");
            return None;
        }

        let _content1 = FileManager::read_file(file1.clone());
        let _content2 = FileManager::read_file(file2.clone());
        let result = "";

        // TODO: finish this function

        return Option::from(result.to_string());
    }

    pub fn read_file(file_name: String) -> Option<String> {
        if !FileManager::path_exists(file_name.clone()) {
            println!("File doesnt exist!");
            return None;
        }

        return Option::from(fs::read_to_string(file_name).expect(
            "Couldn't read file."));
    }

    pub fn clone(path1: String, path2: String) -> bool {
        if !FileManager::path_exists(path1.clone()) {
            println!("Clone path doesnt exist!");
            return true;
        }
        if FileManager::path_exists(path2.clone()) {
            println!("Cloned path already exist!");
            return true;
        }

        let path1_ = path1.clone();
        let path2_ = path2.clone();

        if option2bool(FileManager::is_dir(&path1_)) {
            copy_dir(&path1_, &path2_).expect("Couldn't copy directory.");
        }
        else {
            fs::File::create(&path2_).expect("Couldn't copy file.");
        }

        return false;
    }

    pub fn is_dir(path: &String) -> Option<bool> {
        if !FileManager::path_exists(path.clone()) {
            println!("Path doesnt exist!");
            return None;
        }

        let metadata = fs::metadata(&path).unwrap();

        return Option::from(metadata.is_dir());
    }

    pub fn remove(path: String) -> bool {
        if !FileManager::path_exists(path.clone()) {
            println!("Path doesnt exist!");
            return true;
        }

        let path_ = path.clone();

        if option2bool(FileManager::is_dir(&path_)) {
            // remove_dir_all also removes files in that directory (not just directory)
            fs::remove_dir_all(&path_).expect("Couldn't remove directory.");
        }
        else {
            fs::remove_file(&path_).expect("Couldn't remove file.");
        }

        return false;
    }

    pub fn touch(path: String, is_file: bool) -> bool {
        if FileManager::path_exists(path.clone()) {
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
        if !FileManager::path_exists(path1.clone()) {
            println!("Rename path doesnt exist!");
            return true;
        }
        if FileManager::path_exists(path2.clone()) {
            println!("Renamed path already exist!");
            return true;
        }

        fs::rename(path1, path2).expect("Couldn't rename file or directory.");

        return false;
    }

    pub fn get_working_directory() -> String {
        let path = env::current_dir().expect("Couldn't get working directory.");
        return path.display().to_string();
    }
}
