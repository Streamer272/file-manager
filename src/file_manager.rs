use copy_dir::copy_dir;
use std::fs;


pub fn optionstr_2_string(text: Option<&str>) -> String {
    return String::from(text.unwrap());
}

pub struct FileManager;

impl FileManager {
    // TODO: add exception handling and file exists checking

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
        let path1_ = path1.clone();
        let path2_ = path2.clone();

        if FileManager::is_dir(&path1_) {
            copy_dir(&path1_, &path2_).expect("Couldn't copy directory.");
        }
        else {
            fs::File::create(&path2_).expect("Couldn't copy file.");
        }
    }

    pub fn is_dir(path: &String) -> bool {
        let metadata = fs::metadata(&path).unwrap();

        return metadata.is_dir();
    }

    pub fn remove(path: String) {
        let path_ = path.clone();

        if FileManager::is_dir(&path_) {
            // remove_dir_all also removes files in that directory (not just directory)
            fs::remove_dir_all(&path_).expect("Couldn't remove directory.");
        }
        else {
            fs::remove_file(&path_).expect("Couldn't remove file.");
        }
    }

    pub fn touch(path: String, is_file: bool) {
        if is_file {
            fs::File::create(path).expect("Couldn't create file.");
        }
        else {
            fs::create_dir(path).expect("Couldn't create directory.");
        }
    }

    pub fn rename(path1: String, path2: String) {
        fs::rename(path1, path2).expect("Couldn't rename file or directory.");
    }
}
