use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::fmt;
use std::io::Read;
use std::process::Command;
use std::path::Path;
use serde_yaml;
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
    pub path: Option<String>,
    pub md5: Option<String>,
    pub size: Option<i64>,
    pub desc: Option<String>,
    pub remote: Option<String>,
}

pub fn parse_dvc_data_registry(dvc_dir: &Path) -> Vec<DataCategory>{
    fn parse_dvc_file(file_path: &Path) -> DataCategory {
        let mut data_category = DataCategory {
            path: None,
            md5: None,
            size: None,
            desc: None,
            remote: None,
        };
    
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
    
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() == 2 {
                        let key = parts[0];
                        let value = parts[1];
    
                        match key {
                            "- path" => data_category.path = Some(value.to_string()),
                            "  md5" => data_category.md5 = Some(value.to_string()),
                            "  size" => data_category.size = value.parse().ok(),
                            "  desc" => data_category.desc = Some(value.to_string()),
                            "  remote" => data_category.remote = Some(value.to_string()),
                            _ => (),
                        }
                    }
                }
            }
        }
        let x = 1;
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

mod tests {
    impl fmt::Debug for DataCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Path: {}", self.path.as_ref().map(String::as_str).unwrap_or_default())?;
        writeln!(f, "MD5: {}", self.md5.as_ref().map(String::as_str).unwrap_or_default())?;
        writeln!(f, "Size: {}", self.size.unwrap_or_default())?;
        writeln!(f, "Desc: {}", self.desc.as_ref().map(String::as_str).unwrap_or_default())?;
        writeln!(f, "Remote: {}", self.remote.as_ref().map(String::as_str).unwrap_or_default())?;
        Ok(())
    }
}
    use super::*;
    #[test]
    fn test_parse_dvc_data_registry() {
        let res = super::parse_dvc_data_registry(&Path::new("dataset-registry"));
        println!("Vector {:?}", res);
    }
}