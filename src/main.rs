use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};

fn get_sub_entries(path: &str) -> Result<HashMap<String, String>, Error> {
    let mut entries = HashMap::new();
    let metadata = fs::metadata(path)?;

    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path_buff = entry.path();
            let parent = path_buff
                .parent()
                .ok_or(Error::new(ErrorKind::NotFound, "Could not find parent!"))?;

            entries.insert(
                path_buff.display().to_string(),
                parent.display().to_string()
            );
        }
    }

    Ok(entries)
}

fn get_directory_structure(path: &str) -> Result<HashMap<String, String>, Error> {
    let mut entries = get_sub_entries(path)?;

    loop {
        let entry_count = entries.len();
        let mut sub_entries = HashMap::new();

        for entry in entries.keys() {
            sub_entries.extend(get_sub_entries(entry)?);
        }

        entries.extend(sub_entries);

        if entry_count == entries.len() {
            break;
        }
    }

    Ok(entries)
}

fn main() {
    let result = get_directory_structure(r#"C:\Users\Jimmy\Desktop\Test"#);

    match result {
        Ok(res) => println!("Hurray!"),
        Err(err) => println!("{:?}", err),
    };
}
