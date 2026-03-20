use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

pub fn get_routing() -> Result<HashMap<String, PathBuf>, std::io::Error> {
    let mut router_map: HashMap<String, PathBuf> = HashMap::new();

    let exe_path = env::current_exe()?;
    let folder_path = exe_path.parent().expect("Invalid parent folder name");
    let folder_to_seek_path = folder_path.join("html");

    if !folder_to_seek_path.exists() {
        eprintln!("Create a html directory first!");
        exit(1);
    }

    for entry in fs::read_dir(folder_to_seek_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let Ok(path_name) = entry.file_name().into_string() else {
                continue;
            };
            let path_vec: Vec<&str> = path_name.split(".").collect();
            let path_name: String;
            if let Some(ext) = path_vec.last() {
                if *ext == "html" {
                    path_name = path_vec[..path_vec.len() - 1].join(".");
                } else {
                    path_name = path_vec.join(".")
                }
            } else {
                continue;
            }

            let file_path = folder_path.join("html").join(entry.file_name());
            router_map.entry(path_name).or_insert(file_path);
        }
    }

    return Ok(router_map);
}
