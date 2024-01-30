use crate::common::*;

pub fn deploy(_path : &String){
   let mut command = Command::new("npx")
        .args(["sf","project","deploy","start","-c","-m","ApexClass","StaticResource","CustomObject","ApexComponent","ApexPage","CustomLabels","Flow","LightningComponentBundle"])
        .env("PATH","~/Documents/SalesForce/Work")
        // .args(["project","deploy","start","--manifest",path])
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
