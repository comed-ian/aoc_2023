use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn read_input(filepath: &str) -> Result<String, std::io::Error> {
	let mut path: PathBuf = workspace_dir();
	path.push(Path::new(filepath));
    read_to_string(path.as_path())
}

pub fn read_lines(filepath: &str) -> Vec<String> {
	let mut path: PathBuf = workspace_dir();
	path.push(Path::new(filepath));
    let file = File::open(path.as_path()).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let dir = "utils/input.txt";
        assert_eq!("hello world\nhello earth",
            read_input(dir).unwrap());
        assert_eq!(vec!["hello world", "hello earth"], read_lines(&dir));
    }
}
