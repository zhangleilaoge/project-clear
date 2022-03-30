use std::env;
use std::fs::{self};
use std::io;
use std::path::Path;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough args");
    }

    let root_path = Path::new(&args[1]);
    let delete_path = &args[2];

    visit_dirs(root_path, delete_path).unwrap_or_default();
}

fn visit_dirs(dir: &Path, delete_path: &String) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let path_str = path.to_string_lossy();
                if path_str.contains(delete_path) {
                    println!("{} has been deleted", path_str);
                    fs::remove_dir_all(String::from(path_str))?;
                } else {
                    visit_dirs(&path, delete_path)?;
                }
            }
        }
    }
    Ok(())
}
