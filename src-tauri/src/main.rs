// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn confirm(paths: Vec<String>, suffix: String) -> Vec<String> {
    use std::fs;
    use std::ffi::OsStr;
    use std::path::Path;
    let mut results = Vec::new();
    let suffix = OsStr::new(&suffix);
    for path in paths {
        if let Ok(entries) = fs::read_dir(Path::new(&path)) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        results.extend(confirm(vec![path.to_string_lossy().into_owned()], suffix.to_str().unwrap().to_string()));
                    } else if let Some(ext) = path.extension() {
                        if ext == suffix && path.file_name().unwrap().to_str().unwrap() != "GEOMODEL.inp" {
                            results.push(path.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }
    results
}

#[tauri::command]
fn start_simulate(inppaths: Vec<String>, selectedpath: String, version: String, cpunumber: String) {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::process::Command;
    use std::path::Path;
    use std::thread;
    use std::os::windows::process::CommandExt;
    use winapi::um::winbase::CREATE_NO_WINDOW;

    thread::spawn(move || {
        print!("Selected path: {}", selectedpath);
        let log_txt_path = Path::new(&selectedpath).join("log.txt");
        let mut log_txt = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&log_txt_path)
            .expect("Failed to open log.txt");

            for (index, path) in inppaths.iter().enumerate() {
                let path = Path::new(&path);
                let filename = path.file_stem().expect("Failed to get file name");
                let inpname = filename.to_str().expect("Failed to convert to string");
            
                let parent_dir = path.parent().expect("Failed to get parent directory");
                println!("Parent directory: {:?}", parent_dir);
            
                writeln!(log_txt, "{}开始模拟", path.display()).expect("Failed to write to log.txt");
            
                let log_ll_path = Path::new(&selectedpath).join("log.ll");
                let mut log_ll = File::create(&log_ll_path).expect("Failed to create log.ll");
                writeln!(log_ll, "{}", index).expect("Failed to write to log.ll");
            
                // Check if a file with the same name but .odb extension exists
                let odb_path = parent_dir.join(format!("{}.odb", inpname));
                if !odb_path.exists() {
                    let mut cmd = Command::new("cmd");
                    cmd.creation_flags(CREATE_NO_WINDOW);
                    cmd.current_dir(parent_dir);
                    cmd.arg("/c");
                    cmd.arg("call");
                    cmd.arg(format!("abq{}", version)); // Use the version parameter here
                    cmd.arg(format!("job={}", inpname));
                    cmd.arg(format!("cpus={}", cpunumber)); // Use the cpunumber parameter here
                    cmd.arg("int");
                    cmd.output().expect("Failed to execute process");
                    println!("Executing command: {:?}", cmd);
                }
            
                writeln!(log_txt, "{}模拟完成", path.display()).expect("Failed to write to log.txt");
            }
    });
}


#[tauri::command]
fn start1(inppaths: Vec<String>) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    let mut file_path = PathBuf::from("G:/Model/Abaqus/project");
    file_path.push("1.txt");

    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;

    for path in inppaths {
        writeln!(file, "{}", path).map_err(|e| e.to_string())?;
    }

    Ok("success".to_string())
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    use encoding_rs::WINDOWS_1252;
    let bytes = std::fs::read(&path).map_err(|err| err.to_string())?;
    let (content, _, _) = WINDOWS_1252.decode(&bytes);
    let last_line = content.lines().last().unwrap_or("").to_string();
    Ok(last_line)
}

#[tauri::command]
fn suspend(dir_path: String, command: String) -> Result<String, String> {
    use std::process::Command;
    use std::env;

    // Change the current directory to the provided directory path
    env::set_current_dir(&dir_path).map_err(|err| err.to_string())?;

    // Execute the command
    let output = Command::new(command)
        .output()
        .map_err(|err| err.to_string())?;

    // Convert the output to a string
    let output_str = String::from_utf8(output.stdout).map_err(|err| err.to_string())?;

    Ok(output_str)
}

#[tauri::command]
fn aftertreat1(macrofile: String, odbpaths: Vec<String>, replace: Vec<Vec<String>>,version: String) -> Result<String, String> {
    use std::fs;
    use std::io::{Read, Write};
    use std::path::Path;

    // thread::spawn(move ||{})

    println!("Macro: {}", macrofile);
    println!("OdbPaths: {:?}", odbpaths);
    println!("Replace: {:?}", replace);
    for odbpath in odbpaths {
        let path = Path::new(&odbpath);
        if let Some(filename) = path.file_stem() {
            println!("File name: {:?}", filename);
            let new_file_path = Path::new(&odbpath).with_extension("py");
            println!("new_file_path: {:?}", new_file_path);
            match fs::copy(&macrofile, &new_file_path) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to copy file: {}", e)),
            };

            let mut file_content = String::new();
            {
                let mut file = match fs::File::open(&new_file_path) {
                    Ok(file) => file,
                    Err(e) => return Err(format!("Failed to open file: {}", e)),
                };
                match file.read_to_string(&mut file_content) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("Failed to read file: {}", e)),
                };
            }

            for pair in &replace {
                let mut new_string = pair[1].clone();
                new_string = new_string.replace("{0}", path.file_name().unwrap().to_str().unwrap());
                if path.parent().is_some() {
                    let parent = path.parent().unwrap();
                    new_string = new_string.replace("{1}", parent.file_name().unwrap().to_str().unwrap());
                    if parent.parent().is_some() {
                        let grand_parent = parent.parent().unwrap();
                        new_string = new_string.replace("{2}", grand_parent.file_name().unwrap().to_str().unwrap());
                        if grand_parent.parent().is_some() {
                            let great_grand_parent = grand_parent.parent().unwrap();
                            new_string = new_string.replace("{3}", great_grand_parent.file_name().unwrap().to_str().unwrap());
                            if great_grand_parent.parent().is_some() {
                                let great_great_grand_parent = great_grand_parent.parent().unwrap();
                                if great_great_grand_parent.file_name().is_some() {
                                    new_string = new_string.replace("{4}", great_great_grand_parent.file_name().unwrap().to_str().unwrap());
                                }
                            }
                        }
                    }
                }
                new_string = new_string.replace("{odb}", &filename.to_str().unwrap());
                new_string = new_string.replace("{pwd}", path.parent().unwrap().to_str().unwrap());
                file_content = file_content.replace(&pair[0], &new_string);
            }

            {
                let mut file = match fs::File::create(&new_file_path) {
                    Ok(file) => file,
                    Err(e) => return Err(format!("Failed to open file: {}", e)),
                };
                match file.write_all(file_content.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("Failed to write file: {}", e)),
                };
            }

            
            let command = format!("call abq{} cae noGUI={}", version, new_file_path.file_name().unwrap().to_str().unwrap());            println!("Working directory: {}", path.parent().unwrap().to_str().unwrap());
            let output = std::process::Command::new("cmd")
                .current_dir(path.parent().unwrap())
                .args(&["/C", &command])
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                return Err(format!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
    }

    Ok("success".to_string())
}


#[tauri::command]
fn aftertreat(macrofile: String, odbpaths: Vec<String>, replace: Vec<Vec<String>>,version: String) -> Result<String, String> {
    use std::fs;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::thread;
    use std::process::{Command, Stdio};
    use std::os::windows::process::CommandExt;
    use winapi::um::winbase::CREATE_NO_WINDOW;


    thread::spawn(move || {
        println!("Macro: {}", macrofile);
        println!("OdbPaths: {:?}", odbpaths);
        println!("Replace: {:?}", replace);

        for odbpath in odbpaths {
            let path = Path::new(&odbpath);
            if let Some(filename) = path.file_stem() {
                println!("File name: {:?}", filename);
                let new_file_path = Path::new(&odbpath).with_extension("py");
                println!("new_file_path: {:?}", new_file_path);
                match fs::copy(&macrofile, &new_file_path) {
                    Ok(_) => (),
                    Err(e) => panic!("Failed to copy file: {}", e),                };
    
                let mut file_content = String::new();
                {
                    let mut file = match fs::File::open(&new_file_path) {
                        Ok(file) => file,
                        Err(e) => panic!("Failed to open file: {}", e),
                    };
                    match file.read_to_string(&mut file_content) {
                        Ok(_) => (),
                        Err(e) => panic!("Failed to read file: {}", e),
                    };
                }
    
                for pair in &replace {
                    let mut new_string = pair[1].clone();
                    new_string = new_string.replace("{0}", path.file_name().unwrap().to_str().unwrap());
                    if path.parent().is_some() {
                        let parent = path.parent().unwrap();
                        new_string = new_string.replace("{1}", parent.file_name().unwrap().to_str().unwrap());
                        if parent.parent().is_some() {
                            let grand_parent = parent.parent().unwrap();
                            new_string = new_string.replace("{2}", grand_parent.file_name().unwrap().to_str().unwrap());
                            if grand_parent.parent().is_some() {
                                let great_grand_parent = grand_parent.parent().unwrap();
                                new_string = new_string.replace("{3}", great_grand_parent.file_name().unwrap().to_str().unwrap());
                                if great_grand_parent.parent().is_some() {
                                    let great_great_grand_parent = great_grand_parent.parent().unwrap();
                                    if great_great_grand_parent.file_name().is_some() {
                                        new_string = new_string.replace("{4}", great_great_grand_parent.file_name().unwrap().to_str().unwrap());
                                    }
                                }
                            }
                        }
                    }
                    new_string = new_string.replace("{odb}", &filename.to_str().unwrap());
                    new_string = new_string.replace("{pwd}", path.parent().unwrap().to_str().unwrap());
                    file_content = file_content.replace(&pair[0], &new_string);
                }
    
                {
                    let mut file = match fs::File::create(&new_file_path) {
                        Ok(file) => file,
                        Err(e) => panic!("Failed to open file: {}", e),
                    };
                    match file.write_all(file_content.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => panic!("Failed to write file: {}", e),
                    };
                }
    
                
                let command = format!("call abq{} cae noGUI={}", version, new_file_path.file_name().unwrap().to_str().unwrap());            println!("Working directory: {}", path.parent().unwrap().to_str().unwrap());
                let output = Command::new("cmd")
                    .current_dir(path.parent().unwrap())
                    .args(&["/C", &command])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .expect("Failed to execute command");
    
                if !output.status.success() {
                    panic!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    
        

    });
    Ok("success".to_string())
    
    // handle.join().unwrap_or_else(|_| Err("Thread panicked".to_string()))
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, confirm,start_simulate,start1,aftertreat,read_file,suspend]) // 注册 confirm 函数   
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

