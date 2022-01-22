use std::{
    fs,
    io::{Error, ErrorKind},
};

fn get_subfolders(path: &str, level: i32) -> Result<Vec<(String, i32)>, Error> {
    if let Ok(entries) = fs::read_dir(path) {
        Ok(entries
            .filter_map(|e| e.ok())
            .map(|e| (e.path().display().to_string(), level))
            .collect())
    } else {
        Err(Error::new(
            
            ErrorKind::NotFound,
            format!("Could not find: {}", path),
        ))
    }
}

fn get_directory_tree(mut directory_tree: Vec<(String, i32)>) ->Vec<(String, i32)> {
    loop {
        let elements_at_start = directory_tree.len();
        let current_max = directory_tree.iter().map(|d| d.1).max().unwrap();

        let mut sub_directories: Vec<(String, i32)> = directory_tree
            .iter()
            .filter(|d| d.1 == current_max)
            .filter_map(|d: &(String, i32)| get_subfolders(&d.0, current_max + 1).ok())
            .flatten()
            .map(|d: (String, i32)| (d.0, d.1))
            .collect();

        let elements_at_end = directory_tree.len() + sub_directories.len();
        directory_tree.append(&mut sub_directories);        

        if elements_at_end == elements_at_start {
            break;
        }
    }
    directory_tree
}

fn main() {
    if let Ok(sufolders) = get_subfolders("/home/jimmy/Desktop/Test", 0) {
        let subfolders = get_directory_tree(sufolders);
        println!("Success!");
    } else {
        println!("Failure!");
    }
    // let subFolders = get_subfolders("C:\\Users\\Jimmy\\Desktop\\Test");
    // println!("Hello World!");
}
