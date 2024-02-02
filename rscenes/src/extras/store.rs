use serde::{de::DeserializeOwned, Serialize};
use std::{
    env,
    fmt::Debug,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct XDGStore;

impl XDGStore {
    pub fn init_storage(app_name: &str) -> Result<(), String> {
        let data_home = app_data_home(app_name);
        if !data_home.exists() {
            fs::create_dir_all(data_home).or_else(|e| Err(format!("{:?}", e)))
        } else {
            Ok(())
        }
    }

    pub fn save<T: Serialize>(app_name: &str, bundle: &str, data: T) -> Result<(), String> {
        let payload = serde_json::to_string(&data).or_else(|e| Err(format!("{:?}", e)))?;
        let mut file = bundle_file(app_name, bundle, RW::RW)?;
        file.write_all(payload.as_bytes())
            .or_else(|e| Err(format!("{:?}", e)))?;
        Ok(())
    }

    pub fn retrieve<T: DeserializeOwned>(app_name: &str, bundle: &str) -> Result<T, String> {
        let mut file = bundle_file(app_name, bundle, RW::RO)?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .or_else(|e| Err(format!("{:?}", e)))?;
        let res: T = serde_json::from_str(&content).or_else(|e| Err(format!("{:?}", e)))?;
        Ok(res)
    }
}

fn app_data_home(app_name: &str) -> PathBuf {
    xdg_data_home().join(app_name)
}

fn bundle_file(app_name: &str, bundle: &str, mode: RW) -> Result<File, String> {
    let mut storage = app_data_home(app_name).join(bundle);
    storage.set_extension("json");
    match mode {
        RW::RO => File::open(storage),
        RW::RW => File::create(storage),
    }
    .or_else(|e| Err(format!("{:?}", e)))
}

enum RW {
    RO,
    RW,
}

#[cfg(not(target_os = "windows"))]
fn xdg_data_home() -> PathBuf {
    match env::var("XDG_DATA_HOME") {
        Ok(data_home) => Path::new(&data_home).to_path_buf(),
        _ => {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(".local").join("share").to_path_buf()
        }
    }
}

#[cfg(target_os = "windows")]
fn xdg_data_home() -> PathBuf {
    let data_home = env::var("APPDATA").unwrap;
    Path::new(&data_home).to_path_buf()
}
