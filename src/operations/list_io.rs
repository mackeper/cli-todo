use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

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
///
/// # Examples
/// ```
/// use cli_todo::list_io::read_file_lines;
///
/// let lines = read_file_lines().unwrap();
/// ```
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

/// Write all lines to the default todo file.
///
/// # Examples
/// ```
/// use cli_todo::list_io::write_file_lines;
///
/// let lines = vec!["foo".to_string(), "bar".to_string()];
/// write_file_lines(&lines).unwrap();
/// ```
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
