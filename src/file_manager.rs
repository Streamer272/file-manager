use copy_dir::copy_dir;
use std::{fs, env};
use difference::{Changeset, Difference};


pub fn option2string(variable: Option<&str>) -> String {
    return String::from(variable.unwrap());
}

pub fn option2bool(boolean: Option<bool>) -> bool {
    return bool::from(boolean.unwrap());
}

pub struct FileManager;

impl FileManager {
    fn path_exists(path: &String) -> bool {
        return fs::metadata(path).is_ok();
    }

    pub fn compare(file1: String, file2: String) -> /* Option<Vec<Difference>> */ Option<Changeset> {
        // TODO: finish this function

        let content1 = match fs::read_to_string(file1) {
            Ok(c) => c,
            Err(_err) => {
                println!("Couldn't read first file!");
                return None;
            }
        };

        let content2 = match fs::read_to_string(file2) {
            Ok(c) => c,
            Err(_err) => {
                println!("Couldn't read second file!");
                return None;
            }
        };

         return Some(Changeset::new(&content1, &content2, "\n"));
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
