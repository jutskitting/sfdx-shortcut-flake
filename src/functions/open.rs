use crate::common::*;

pub fn open(dir:&str){
    println!("{}",dir);
    let mut command = Command::new("npx")
        .current_dir(dir)
        .args(["sf","org","open"])
        .spawn()
        .expect("Failed to execute command");

    let output = command.wait().expect("Failed to wait on child");

    if output.success() {
        println!("Command executed successfully.");
    } else {
        eprintln!("Command failed to execute.");
        if let Some(code) = output.code() {
            eprintln!("Exit code: {}", code);
        } else {
            eprintln!("Command was terminated by a signal.");
        }
    }
}
