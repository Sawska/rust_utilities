use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::env;
use std::fs::{self};
use std::fs::read_to_string;
use std::process::exit;



fn main() {
    let args: Vec<String> = env::args().collect();
    check_command(args);
}

fn check_command(arguments:Vec<String>) {
    let first_command: &String = &arguments[1];
    // println!("{}",first_command);
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
            find(arguments);
        },
        "grep" => {
            grep(arguments);
        }
        _ => {
            error("decision","No such command");
        }
    }
}

fn echo(arguments: Vec<String>) {
    if arguments.len() <= 1 {
        error("Echo", "Not enough arguments");
        return;
    }

    let print_rest = arguments[2..].join(" ");
    println!("{}", print_rest);
}

fn ls(arguments: Vec<String>) {
    let path_str = if arguments.len() <= 1 {
        String::from("./")
    } else {
        arguments[2..].join("")
    };

    let path = PathBuf::from(path_str);
    print_directories(&path);
}

fn print_directories(path: &PathBuf) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        if let Some(name) = file_name.to_str() {
                            println!("{}", name);
                        } else {
                            error("ls", "Encountered non-Unicode file name");
                        }
                    }
                    Err(e) => {
                        error("ls", &format!("Error reading directory: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            error("ls", &format!("Error opening directory: {}", e));
        }
    }
}


fn find(arguments: Vec<String>) {
    if arguments.len() < 3 {
        error("find", "not enough arguments");
        return;
    }

    let file_name = &arguments[3];
    let path = &arguments[4];

    let real_path = locate_file_by_path(file_name, path);

    match real_path {
        Some(path) => println!("Absolute path: {}", path),
        None => println!("File '{}' not found in directory '{}'", file_name, path),
    }
}

fn locate_file_by_path(file_name: &str, path: &str) -> Option<String> {
    let path = Path::new(path);

    if !path.is_dir() {
        error("find", "specified path is not a directory");
        return None;
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let metadata = entry.metadata();
                        if let Ok(metadata) = metadata {
                            if metadata.is_dir() {
                                if let Some(dir_name) = entry.file_name().to_str() {
                                    let dir_path = Path::new(path).join(dir_name);
                                    if let Some(found_path) = locate_file_by_path(file_name, dir_path.to_str().unwrap()) {
                                        return Some(found_path);
                                    }
                                }
                            } else {
                                let name = entry.file_name();
                                if name == file_name {
                                    return Some(entry.path().to_str().unwrap().to_string());
                                }
                            }
                        } else {
                            error("find", "error while checking metadata");
                            continue;
                        }
                    }
                    Err(e) => {
                        error("find", &format!("Error reading directory '{}': {}", path.display(), e));
                        continue;
                    }
                }
            }
            None
        }
        Err(e) => {
            error("find", &format!("Error reading directory '{}': {}", path.display(), e));
            None
        }
    }
}






fn cat(arguments: Vec<String>) {
    if arguments.len() <= 1 {
        error("cat", "not enough arguments");
        return;
    }

    let files = &arguments[2..];
    let mut lines: Vec<String> = Vec::new();
    let current_dir = env::current_dir();
    let binding = current_dir.unwrap();
    let current_path = binding.as_path();

    for file_name in files {
        match locate_file(file_name,current_path.to_str().unwrap()) {
            Some(path) => {
                match print_file(&path) {
                    Ok(content) => lines.push(content),
                    Err(e) => {
                        error("cat", &format!("Error reading file '{}': {}", file_name, e));
                        return;
                    }
                }
            }
            None => {
                error("cat", &format!("File '{}' not found", file_name));
                return;
            }
        }
    }

    for line in lines {
        println!("{}", line);
    }
}

fn locate_file(file_name: &str, current_path: &str) -> Option<String> {
    let entries_result = fs::read_dir(current_path);

    match entries_result {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let name = entry.file_name();
                        if name == file_name {
                            return Some(entry.path().to_string_lossy().to_string());
                        }
                    }
                    Err(_) => continue,
                }
            }
            None
        }
        Err(e) => {
            error("cat", &format!("Error reading directory: {}", e));
            None
        }
    }
}

fn print_file(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
fn grep(arguments: Vec<String>) {
    if arguments.len() < 3 {
        error("grep", "not enough arguments");
        return;
    }

    let text = &arguments[3];
    let file_name = &arguments[4];

    let current_dir = env::current_dir().unwrap_or_else(|e| {
        error("grep", &format!("Error getting current directory: {}", e));
        std::process::exit(1);
    });

    let path = locate_file(file_name, &current_dir.to_str().unwrap());
    let file_path = match path {
        Some(p) => p,
        None => {
            println!("File '{}' not found in directory '{}'", file_name, current_dir.display());
            return;
        }
    };

    let lines = check_if_contains(&file_path, text);

    

    for line in lines {
        println!("{}",line);
    }
}

fn check_if_contains(file_path: &str, text: &str) -> Vec<String> {
    let file_content = read_to_string(file_path).unwrap_or_else(|e| {
        error("grep", &format!("Error reading file '{}': {}", file_path, e));
        std::process::exit(1);
    });

    let mut lines:Vec<String> = Vec::new();

    for line in file_content.lines() {
        if line.contains(text) {
            lines.push(line.to_string());
        }
    }
    lines
}



fn error(command_name:&str,error:&str) {
    eprintln!("Error [{}]: {}", command_name, error);
    exit(0);
}