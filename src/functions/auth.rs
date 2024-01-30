use crate::common::*;

pub fn auth() -> Result<ExitStatus, anyhow::Error>{
    let mut child = Command::new("npx")
        .args(["sf","org","login","web","-r https://test.salesforce.com","-s"])
        .spawn()?;
    Ok(child.wait()?)
}
