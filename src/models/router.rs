
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::env;


pub fn get_routing() -> Result<HashMap<String, PathBuf>, std::io::Error> {
    let mut router_map: HashMap<String, PathBuf> = HashMap::new();
    
    let exe_path = env::current_exe()?;
    let folder_path = exe_path.parent().expect("Invalid parent folder name");
    let folder_to_seek_path = folder_path.join("html");
    
    for entry in fs::read_dir(folder_to_seek_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let path_name = entry.file_name().into_string().expect("Invalid file name");
            let mut path_name = path_name.split(".");
            let path_name = path_name.next()
                                            .ok_or(RouterError::InvalidFileName)
                                            .map_err(|e|std::io::Error::new(std::io::ErrorKind::InvalidFilename, e))?
                                            .to_string();

            let file_path = folder_path.join("html").join(entry.file_name());
            router_map.entry(path_name).or_insert(file_path);
        }
    }
    
    return Ok(router_map);
}

#[derive(Debug)]
pub enum RouterError {
    InvalidFileName
}

impl std::error::Error for RouterError {}
impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
