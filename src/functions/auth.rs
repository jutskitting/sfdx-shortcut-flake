use crate::common::*;


pub fn auth(dir:&str) -> Result<ExitStatus, io::Error>{

    println!("working dir : {}",dir);

    let mut child = Command::new("npx")
        .current_dir(dir)
        .args(["sf","org","login","web","-r https://test.salesforce.com","-s"])
        .env("PATH","/home/kit/Documents/SalesForce/Work")
        .spawn()?;

    child.wait()

}
