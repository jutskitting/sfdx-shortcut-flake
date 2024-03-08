use regex::Regex;
use crate::common::*;

fn class_name(directory : &String) -> Result<String> {

    println!("directory : {}",directory);
    let content = fs::read_to_string(directory)?;

    let re = Regex::new(r"(?si)@isTest(.*?)\{").unwrap();
    if let Some(caps) = re.captures(&content) {
        return Ok(parse_capture(&caps[1]));
    } else {
        return Err(anyhow!("Missing attribute: ClassName"));
    }

}

fn parse_capture(capture : &str) -> String{
    let owned_capture = capture.trim().to_owned();
    let owned_capture = owned_capture.split(" "); 
    return  owned_capture.last().unwrap().to_string();
}

pub fn test(directory : &String) -> Result<ExitStatus,anyhow::Error>{
    println!("starting the test");

    let class =  class_name(directory)?;

    println!("class name : {}",&class);

    let mut command = Command::new("npx")
        .args(["sf","apex","run","test","--result-format","human","--code-coverage","--synchronous","--class-names",&class])
        .spawn()?;

    Ok(command.wait()?)
}
