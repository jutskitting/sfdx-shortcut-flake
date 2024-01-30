use crate::common::*;

pub fn auth(dir:&str) -> Result<ExitStatus, io::Error>{

    println!("working dir : {}",dir);

    let mut child = Command::new("npx")
        .args(["sf","org","login","web","-r https://test.salesforce.com","-s"])
        .spawn()?;

    child.wait()

}
