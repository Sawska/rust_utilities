use std::io::{stdin,stdout,Write};
use std::env;
use std::fs::{self, DirEntry};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    check_command(args);
}

fn check_command(arguments:Vec<String>) {
    let mut first_command = &arguments[0];


    

    match  first_command.as_str() {
        "echo" => {
            echo(arguments);
        },
        "cat" => {
            cat(arguments);
        },
        "ls" => {
            ls(arguments);
        }
        "find" => {

        },
        "grep" => {

        }
        _ => {
            error();
        }
    }
}

fn echo(arguments:Vec<String>) {
    if(arguments.len() == 1)
    {
        error();
    }

    let print_rest = &arguments[1..arguments.len()].to_owned().concat();
    println!("{}",print_rest);
}

fn ls(arguments:Vec<String>) {
    let mut path = String::new();

    if arguments.len() == 1 {
        path.push_str("./");
    } else {
        path.push_str(&arguments[1..arguments.len()].to_owned().concat());
    }
    print_directories(path)
}

fn print_directories(path:String) {
    let mut ls_set = String::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) =>  ls_set.push_str(entry.file_name().to_str().unwrap()),
                    Err(e) => error(),
                }
            }
        }
        Err(e) => error()
    }
}

fn cat(arguments:Vec<String>) {
    if arguments.len() == 1 {
        error();
    } 
    let files = arguments[1..arguments.len()].to_owned();
    let mut lines: Vec<String> = Vec::new();
    for i in 0..files.len() {
        let line = locate_file(&files[i]).unwrap();
        lines.push(line);
    }
}

fn locate_file(file_name: &str) -> Option<String> {
    match fs::read_dir("./") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry.file_name();
                        if name == file_name {
                            return Some(print_file(name.to_str().unwrap()));
                        }
                    }
                    Err(_) => continue, 
                }
            }
            None
        }
        Err(_) => None,
    }
}

fn print_file(file_name:&str) -> String {
    let mut lines:Vec<String> = Vec::new();
    for line in read_to_string(file_name).unwrap().lines() {
        lines.push(line.to_string())
    }
    lines.join("")
}

fn error() {

}