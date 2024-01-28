use crate::common::*;

fn parse_branch(str : String) -> String{
    let ret : String;
    if str.contains("DevKH-"){
        return str.trim().to_owned();
    }else{
        ret = "DevKH-".to_string() + &str;
        return ret.trim().to_owned();
    }
}

fn parse_project(str : &String) -> Option<String>{
    match str.as_str(){
        "ss"=>{
            return Some("Simple-Survey".to_string());
        }
        "cf"=>{
            return Some("Case-Flags".to_string());
        }
        "e2cp"=>{
            return Some("Email-to-Case-Premium".to_string());
        }
        "cs"=>{
            return Some("Case-Split".to_string());
        }
        _=>{
            return None;
        }
    }
}

pub fn ginit(branch:String,project:String,debug:bool){
    let branch_name = parse_branch(branch);
    let project_name;
    if let Some(res) = parse_project(&project.trim().to_owned()){
        if debug {
            println!("result output : {}",res);
        }
       project_name = res; 
    }else{
        if debug {
            println!("given project name : {}",project);
        }
        println!("Didn't match a project name. Let's try something else!");
        return;
    };

    if debug {
        println!("git command : clone --branch {} -single-branch git@github.com:VicassoDev/{}.git",&branch_name, &project_name);
    }

    let mut command = Command::new("git")
        .args(["clone","--branch",&branch_name,"--single-branch",format!("git@github.com:VicassoDev/{}.git",project_name).as_str()])
        .spawn()
        .expect("Failed to execute command");
    let output = command.wait().expect("Failed to wait on child");

    let mut cd = Command::new("cd")
        .arg(&branch_name)
        .spawn()
        .expect("Failed to execute command");
    let _outputcd = cd.wait().expect("Failed to wait on child");

    if output.success() && debug {
        println!("Command executed successfully.");
    } else if debug {
        eprintln!("Command failed to execute.");
        if let Some(code) = output.code() {
            eprintln!("Exit code: {}", code);
        } else {
            eprintln!("Command was terminated by a signal.");
        }
    }
    
}

// # elif [ $Input == 'ginit' ]
// # 	 then
// # 		echo 'branch name'
// # 		read Name
// #         echo 'sub project name'
// # 		read Input
// #         result=$(checkString $Input)
// #         echo 'ok?'
// #         echo $result
// #         git clone --branch $Name --single-branch git@github.com:VicassoDev/${result}.git
// #         cd $result
