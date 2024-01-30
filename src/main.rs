mod common;
mod functions;

use crate::functions::{
    auth::auth,
    open::open,
    grep::grep_find,
    ginit::ginit,
    deploy::deploy,
    test::test,
};

use crate::common::*;


fn main() {

   let current_dir = env::current_dir()
                    .expect("Failed to get current directory");

   let dir_string = current_dir
                    .to_str()
                    .expect("Failed to convert directory to string");

    println!("Current directory : {}",dir_string);

    let args: Vec<String> = env::args().collect();

    let mut has_file = true; 

    if args.len() < 2 {
        has_file = false;
    }

    let mut metadata_path = String::new();
    let mut file_path = &String::new();

    if has_file {
        file_path = &args[1];
        let _main_path = get_main_path(&file_path,"main"); 
        metadata_path = "./package/package.xml".to_owned();
        println!("Received file path: {}", file_path);
    }

    println!("Choose your command :");

    loop {

        println!("exit auth deploy retrieve test open ginit curl info grep");

        let mut pattern = String::new();
        io::stdin().read_line(&mut pattern).expect("Failed to read line");

        println!("Running command: {}",pattern);

        match to_lower_case(&pattern).as_str() {
            "auth" =>{
                handle_response(auth());
            }
            "open" =>{
                handle_response(open()); 
            }
            "deploy" =>{
                if has_file {
                    deploy(&metadata_path);
                }
            }
            "soql" =>{

            }
            "retrieve" =>{

            }
            "test" =>{
                handle_response(test(&file_path));   
            }
            "ginit" =>{
                println!("branch name: DevKH");
                let mut branch = String::new();
                io::stdin().read_line(&mut branch).expect("Failed to read line");

                println!("sub project name : ss");
                let mut project = String::new();
                io::stdin().read_line(&mut project).expect("Failed to read line");

                ginit(branch,project,false);
            }
            "curl" =>{

            }
            "info" =>{

            }
            "grep" =>{
                grep_find();    
            }
            "exit" | "q" | "quit" =>{
                break;
            }
            _ =>{
                println!("Please enter a valid string");
            }

        }
    }
}

fn to_lower_case(string : &String)-> String{
    return string.trim().to_lowercase(); 
}

fn get_main_path(path : &String, segment: &str) -> String{
    let segments: Vec<&str> = path.split('/').collect();
    let mut result = String::new();

    for &s in &segments {
        if !s.is_empty() {
            result.push('/');
            result.push_str(s);
        }
        if s == segment {
            break;
        }
    }
    result
}

fn handle_response(res:Result<std::process::ExitStatus, anyhow::Error>){
    match res {
        Ok(status) => {
            println!("status returned : {}",status);
        },
        Err(err) => {
            println!("error occurred : {}",err);
        }
    }
}
