use rocket::Data;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::process::Command;
use std::path::Path;
use serde_yaml;

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

#[derive(Serialize,Deserialize)]
struct DataCategory{
    md5: String,
    size: i64,
    nfiles: i32,
    path: String
}

fn parse_dvc_data_registry(dvc_dir: &Path) -> Vec<DataCategory>{
    fn parse_dvc_file(file_path: &Path) -> DataCategory {
        let mut file = fs::File::open(file_path).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");

        let data_category: DataCategory = serde_yaml::from_str(&content).expect("Failed to parse YAML");
        return data_category

    }
    let mut dvc_datasets: Vec<DataCategory> = Vec::new();

    let mut recursive_dir_search = || {
        if let Ok(entries) = fs::read_dir(dvc_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        parse_dvc_data_registry(&path);
                    } else if let Some(extension) = path.extension() {
                        if extension == "dvc" {
                            let dvc_dataset = parse_dvc_file(&path);
                            dvc_datasets.push(dvc_dataset)
                        }
                    }
                }
            }
        }
    };
    recursive_dir_search();
    return dvc_datasets
}