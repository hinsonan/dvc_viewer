use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Serialize)]
pub struct FileListing{
    pub files: Vec<String>
}

#[derive(rocket::FromForm)]
pub struct FormData{
    pub name: String,
    pub age: i8
}

pub fn display_files(dir_path: &str) -> Vec<String>{
    let mut string_list: Vec<String> = Vec::new();
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        string_list.push(file_name);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }
    return string_list;
}

pub fn git_clone(git_repo: String){
    let output: std::process::Output = Command::new("git")
        .args(&["clone", &git_repo])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Git clone successful");
    } else {
        let error_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
        println!("Git clone failed: {}", error_message);
    }
}

pub fn parse_dvc_data_registry(dvc_dir:String){
    #[derive(Serialize)]
    struct DataCategory{
        md5: String,
        size: i64,
        nfiles: i32,
        path: String
    }

    
}