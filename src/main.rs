use clap::Parser;
use std::fs::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn list_files(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut out: Vec<String> = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            out.push(path.to_string_lossy().into_owned());
        } else if path.is_dir() {
            out.extend(list_files(&path)?);
        }
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", args.path);
        return Err("Path does not exist".into());
    }

    match list_files(path) {
        Ok(list) => println!("{:#?}", list),
        Err(err) => eprintln!("Error listing files: {}", err),
    }
    Ok(())
}
