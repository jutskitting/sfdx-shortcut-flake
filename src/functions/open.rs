use crate::common::*;

pub fn open() -> Result<ExitStatus, io::Error>{
    let mut child = Command::new("npx")
        .args(["sf","org","open"])
        .spawn()?;
    child.wait()
}
