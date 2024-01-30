use std::fs;
use regex::Regex;
use crate::common::*;

fn class_name(directory : &String) -> Option<String> {
    let content = fs::read_to_string(directory)
        .expect("Could not read file");

    let re = Regex::new(r"(?si)@isTest(.*?)\{").unwrap();
    if let Some(caps) = re.captures(&content) {
        return Some(parse_capture(&caps[1]));
    } else {
        println!("no match!");
    }

    return None;
}

fn parse_capture(capture : &str) -> String{
    let owned_capture = capture.trim().to_owned();
    let owned_capture = owned_capture.split(" "); 
    return  owned_capture.last().unwrap().to_string();
}

pub fn test(directory : &String){
    
    if let Some(class) =  class_name(directory) {

        let mut command = Command::new("npx")
            .args(["sf","apex","run","test","--result-format","human","--code-coverage","--synchronous","--class-names",&class])
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
}
