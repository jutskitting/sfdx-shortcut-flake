use crate::common::*;

fn find_directory(mut current_dir: PathBuf) -> Result<PathBuf, anyhow::Error> {
    loop {
        if current_dir.ends_with("main") || current_dir.ends_with("src") {
            return Ok(current_dir);
        }

        if !current_dir.pop() { 
            return Err(anyhow!("No source or main directory found"));
        }
    }
}


pub fn grep_find(locally:bool,test:bool) -> Result<Option<String>,anyhow::Error> {

    let current_dir = env::current_dir()?;

    match locally{
        true => {
            match find_directory(current_dir) {
                Ok(target_dir) => {
                    println!("Found directory: {:?}", target_dir);
                    env::set_current_dir(&target_dir)?;

                    println!("Enter your grep pattern:");
                    let mut pattern = String::new();
                    io::stdin().read_line(&mut pattern)?;
                    let pattern = pattern.trim();
                    match test {
                        true => {
                            let res = grep_test(target_dir,pattern.to_owned());
                            Ok(Some(res?)) 
                        }
                        false => {
                            let _ = grep(target_dir,pattern.to_owned());
                            Ok(None) 
                        }

                    }
                },
                Err(err) => {
                    println!("{}",err);
                    println!("would you like to search locally? y/n");


                    let mut pattern = String::new();
                    let mut answer = String::new();

                    io::stdin().read_line(&mut answer)?;
                    let answer = answer.trim();
                    match answer {
                        "y" => {
                                let current_dir = env::current_dir()?;
                                io::stdin().read_line(&mut pattern)?;
                                let pattern = pattern.trim();
                                let _ = grep(current_dir,pattern.to_owned());
                        }
                        _ => {
                        }
                    }
                    Ok(None)
                }
            }
        }
        false => {
            println!("using : {}", fs::read_to_string(&current_dir)?);
            println!("Enter your grep pattern:");
            let mut pattern = String::new();
            io::stdin().read_line(&mut pattern)?;
            let pattern = pattern.trim();

            let _ = grep(current_dir,pattern.to_owned());

            Ok(None)
        }
    }
}

fn grep(directory:PathBuf,pattern:String)  -> Result<(),anyhow::Error> {
    let output = Command::new("grep")
        .arg("-R")
        .arg(pattern)
        .current_dir(&directory)
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn grep_test(directory:PathBuf,pattern:String)  -> Result<String,anyhow::Error> {

    // find /path/to/search -type f -iname '*test*' -print0 | xargs -0 grep -l '@isTest'
    //
    let fs_name = fs::read_to_string(&directory)?;
    let output = Command::new("grep")
        .args(["find",&fs_name,"-type","f","-iname",parse_pattern(&pattern).as_str(),"-print0","|","xargs","-0","grep","-l","'@isTest'"])
        .arg(pattern)
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(combine_path("ok".to_string(),"ok".to_string()))
}

fn parse_pattern(pattern:&String)-> String {
    "*".to_string() + &pattern + "*"
}

fn combine_path(path:String,file:String) -> String {
    return "ok".to_string();
}


