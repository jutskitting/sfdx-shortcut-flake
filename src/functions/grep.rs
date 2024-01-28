use crate::common::*;
use std::path::PathBuf;
use std::process::Command;

fn find_directory(mut current_dir: PathBuf) -> Option<PathBuf> {
    loop {
        if current_dir.ends_with("main") || current_dir.ends_with("src") {
            return Some(current_dir);
        }

        if !current_dir.pop() { 
            return None;
        }
    }
}

pub fn grep_find(){
    let current_dir = env::current_dir().expect("Failed to get current directory");

    match find_directory(current_dir) {
        Some(target_dir) => {
            println!("Found directory: {:?}", target_dir);
            env::set_current_dir(&target_dir).expect("Failed to change directory");

            println!("Enter your grep pattern:");
            let mut pattern = String::new();
            io::stdin().read_line(&mut pattern).expect("Failed to read line");
            let pattern = pattern.trim();

            let output = Command::new("grep")
                .arg("-R")
                .arg(pattern)
                .current_dir(&target_dir)
                .output()
                .expect("Failed to execute grep");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        },
        None => {
            println!("Neither 'main' nor 'src' directory found");
        }
    }
}
