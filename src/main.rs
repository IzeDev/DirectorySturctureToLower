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

    if let Ok(res) = result  {
        let mut entries: Vec<(String, String)> = res
            .into_iter()
            .map(|(item, parent)| (item.as_str().replace(&parent, ""), parent, item))
            .map(|(item, parent, original_item)| (parent + &item.to_lowercase(), original_item))
            .collect();

        // dummies.sort_by(|d1, d2| d1.x.cmp(&d2.x));
        for entry in &entries {
            println!("Original: {}, new: {}", entry.1, entry.0)
        }

        println!("----");
        
        let y = 5;

        entries.sort_by(|i1, i2| i2.1.cmp(&i1.1));
        

        for entry in &entries {
            println!("Original: {}, new: {}", entry.1, entry.0)
        }
            
        let y = 5;

    }
    else if let Err(err) = result {
        println!("{:?}", err)
    }
}
