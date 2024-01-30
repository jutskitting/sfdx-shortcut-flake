use crate::common::*;

pub fn auth() -> Result<ExitStatus, io::Error>{
    let mut child = Command::new("npx")
        .args(["sf","org","login","web","-r https://test.salesforce.com","-s"])
        .spawn()?;
    child.wait()
}
