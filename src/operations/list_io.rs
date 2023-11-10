use std::fs::OpenOptions;
use std::io::{self, BufReader, Write, Read};
use std::path::PathBuf;

use crate::domain::List;

fn get_file_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let mut path = PathBuf::from(std::env::var("LOCALAPPDATA").unwrap());
        path.push("cli-todo");
        std::fs::create_dir_all(&path).unwrap();
        path.push("default-todo.txt");
        path
    } else {
        PathBuf::from("default-todo.txt")
    }
}

/// Read all lines from the default todo file.
pub fn read_file_lines() -> io::Result<List> {
    let path = get_file_path();
    let file = OpenOptions::new()
        .read(true)
        .write(true) // Needed to create in windows?
        .create(true)
        .open(path)?;
    let mut json_list = String::new();
    BufReader::new(file).read_to_string(&mut json_list)?;
    let list = match serde_json::from_str(&json_list) {
        Ok(result) => { result },
        Err(_) => { List::new("default".to_string()) },
    };
    Ok(list)
}

/// Write all lines to the default todo file.
pub fn write_file_lines(list: &List) -> io::Result<()> {
    let path = get_file_path();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let json_list = serde_json::to_string(list)?;
    file.write_all(json_list.as_bytes())?;
    Ok(())
}
