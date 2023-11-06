use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

fn get_file_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let mut path = PathBuf::from(std::env::var("LOCALAPPDATA").unwrap());
        path.push("cli-todo");
        std::fs::create_dir_all(&path).unwrap();
        path.push("default.txt");
        path
    } else {
        PathBuf::from("todo.txt")
    }
}

pub fn read_file_lines() -> io::Result<Vec<String>> {
    let path = get_file_path();
    let file = OpenOptions::new()
        .read(true)
        .write(true) // Needed to create in windows?
        .create(true)
        .open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|l| l.ok()).collect())
}

pub fn write_file_lines(lines: &[String]) -> io::Result<()> {
    let path = get_file_path();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
