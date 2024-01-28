use crate::common::*;

pub fn auth(dir:&str){
   let mut command = Command::new("npx")
        .current_dir(dir)
        .args(["sf","org","login","web","-r https://test.salesforce.com","-s"])
        .env("PATH","~/Documents/SalesForce/Work")
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

