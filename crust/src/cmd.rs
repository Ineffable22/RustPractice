pub mod commands {
    use std::process::Command;

    pub fn execve(token: Vec<&str>) {
        let cmd: String = token[0].to_string();
        let mut args: Vec<&str> = Vec::new();
        for i in 1..token.len() {
            args.push(token[i]);
        }
        match Command::new(cmd).args(args).status() {
            Ok(_) => (),
            Err(_) => println!("{}: command not found", token[0]),
        }
    }
}

pub mod built_in {
    use std::env::set_current_dir;
    use std::process::exit as to_exit;
    pub fn exit(token: Vec<&str>) {
        if token.len() > 2 {
            println!("exit: too many arguments");
        } else if token.len() == 1 {
            to_exit(0);
        } else {
            to_exit(token[1].parse::<i32>().unwrap());
        }
    }

    pub fn cd(token: Vec<&str>) {
        if token.len() > 2 {
            println!("cd: too many arguments");
        } else if token.len() == 1 {
            match set_current_dir(dirs::home_dir().unwrap()) {
                Ok(_) => (),
                Err(_) => println!("cd: no such file or directory: {}", token[1]),
            }
        } else {
            match set_current_dir(token[1]) {
                Ok(_) => (),
                Err(_) => println!("cd: no such file or directory: {}", token[1]),
            }
        }
    }
}
