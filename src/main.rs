use std::{
    fs,
    io::{Error, ErrorKind},
};

fn get_subfolders(path: &str) -> Result<Vec<String>, Error> {
    if let Ok(entries) = fs::read_dir(path) {
        Ok(entries
            .filter_map(|e| e.ok())
            .map(|e| e.path().display().to_string())
            .collect())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Could not find: {}", path),
        ))
    }
}

fn get_directory_tree(directory_tree: Vec<(String, i32)>) {
    let mut number_of_directories = 1;

    while number_of_directories < directory_tree.len() {
        let current_level = 0;

        let x = &directory_tree
            .into_iter()
            .filter_map(|d: (String, i32)| get_subfolders(&d.0).ok())
            .fold(init: Acc, fold: Fold)
            // .map(|d| (d, current_level + 1))
            // .collect();
    }
}

fn main() {
    if let Ok(sufolders) = get_subfolders("C:\\Users\\Jimmy\\Desktop\\Test") {
        println!("Success!");
        for subfolder in &sufolders {
            println!("{}", subfolder);
        }
        sufolders.into_iter().for_each(|f| println!("{}", f));
    } else {
        println!("Failure!");
    }
    // let subFolders = get_subfolders("C:\\Users\\Jimmy\\Desktop\\Test");
    // println!("Hello World!");
}
