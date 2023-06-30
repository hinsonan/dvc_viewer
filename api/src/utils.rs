use rocket::serde::{Serialize};
use std::fs;
use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Serialize)]
pub struct DataCategory{
    pub title: Option<String>,
    pub path: Option<String>,
    pub md5: Option<String>,
    pub size: Option<i64>,
    pub nfiles: Option<i64>,
    pub desc: Option<String>,
    pub remote: Option<String>,
}

pub fn parse_dvc_data_registry(dvc_dir: &Path) -> Vec<DataCategory>{
    fn parse_dvc_file(file_path: &Path) -> DataCategory {
        let mut data_category = DataCategory {
            title: None,
            path: None,
            md5: None,
            size: None,
            nfiles: None,
            desc: None,
            remote: None,
        };
        
        if let Some(title_str) = file_path.to_str() {
            data_category.title = Some(title_str.to_owned());
        }
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
    
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1];
    
                        match key {
                            "- path" => data_category.path = Some(value.trim().to_string()),
                            "md5" => data_category.md5 = Some(value.trim().to_string()),
                            "size" => data_category.size = value.parse().ok(),
                            "nfiles" => data_category.nfiles = value.parse().ok(),
                            "desc" => data_category.desc = Some(value.trim().to_string()),
                            "remote" => data_category.remote = Some(value.trim().to_string()),
                            _ => (),
                        }
                    }
                }
            }
        }
        return data_category
    }
    let mut dvc_datasets: Vec<DataCategory> = Vec::new();

    let mut recursive_dir_search = || {
        if let Ok(entries) = fs::read_dir(dvc_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let sub_datasets = parse_dvc_data_registry(&path);
                        dvc_datasets.extend(sub_datasets);
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
