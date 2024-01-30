use crate::common::*;

pub fn open() -> Result<ExitStatus,anyhow::Error>{
    let mut child = Command::new("npx")
        .args(["sf","org","open"])
        .spawn()?;
    Ok(child.wait()?)
}
