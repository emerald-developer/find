use std::fs::*;

fn list_files(path: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(enteries) = read_dir(path) {
        for entry in enteries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        out.push(path.to_string_lossy().into_owned());
                    } else if path.is_dir() {
                        out.extend(list_files(&path.to_string_lossy()));
                    }
                }
                Err(err) => {
                    eprintln!("Error reading directory entry: {}", err);
                }
            }
        }
    }
    out
}

fn main() {
    println!("Hello, world!");
    let list: Vec<String> = list_files("./test");
    println!("{:#?}", list);
}
